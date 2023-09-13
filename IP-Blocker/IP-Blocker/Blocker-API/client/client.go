package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"sync"

	pb "Blocker_API/proto/proto" // Update the import path
	"google.golang.org/grpc"
)

var (
	ipInfos       []IPInfo
	latestIPLock  = &sync.Mutex{}
	updateChannel = make(chan string)
	ipInfosLock   = &sync.Mutex{}
)

type IPInfo struct {
	IP   string `json:"ip"`
	Port string `json:"port"`
}

func watchForIPUpdates() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		fmt.Printf("Failed to connect to server: %v\n", err)
		return
	}
	defer conn.Close()

	client := pb.NewIPServiceClient(conn)

	stream, err := client.WatchIPUpdates(context.Background(), &pb.EmptyRequest{})
	if err != nil {
		fmt.Printf("Failed to open IP updates stream: %v\n", err)
		return
	}

	for {
		ipUpdate, err := stream.Recv()
		if err != nil {
			fmt.Printf("Error receiving IP update: %v\n", err)
			return
		}

		ip := ipUpdate.IpAddress
		fmt.Printf("Received IP update from server: %s\n", ip)

		if isValidIP(ip) && !isIPInfoExists(ip) {
			ipInfosLock.Lock() // Acquire the lock before modifying the slice
			ipInfos = append(ipInfos, IPInfo{
				IP:   ip,
				Port: "8080", // Default port
			})
			ipInfosLock.Unlock() // Release the lock after modifying the slice
			writeIPInfosToFile()
		}
	}
}

func isValidIP(ip string) bool {
	return strings.TrimSpace(ip) != ""
}

func isIPInfoExists(ip string) bool {
	ipInfosLock.Lock() // Acquire the lock before accessing the slice
	defer ipInfosLock.Unlock() // Ensure the lock is released when the function exits

	for _, info := range ipInfos {
		if info.IP == ip {
			return true
		}
	}
	return false
}

func writeIPInfosToFile() {
	//fileName := "../Policy/output.json"
	fileName := "/usr/src/app/Policy/output.json"
	file, err := os.Create(fileName)
	if err != nil {
		fmt.Printf("Failed to create file: %v\n", err)
		return
	}
	defer file.Close()

	ipInfosLock.Lock() // Acquire the lock before accessing the slice
	defer ipInfosLock.Unlock() // Ensure the lock is released when the function exits

	enc := json.NewEncoder(file)
	enc.SetIndent("", "  ") // Set indentation for pretty formatting

	if err := enc.Encode(ipInfos); err != nil {
		fmt.Printf("Failed to encode JSON: %v\n", err)
	}
}

func main() {
	go watchForIPUpdates()

	select {} // Keep the main goroutine running
}




/*
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"sync"

	pb "Blocker_API/proto/proto" // Update the import path
	"google.golang.org/grpc"
)

var (
	ipInfos       []IPInfo
	latestIPLock  = &sync.Mutex{}
	updateChannel = make(chan string)
)

type IPInfo struct {
	IP   string `json:"ip"`
	Port string `json:"port"`
}

func watchForIPUpdates() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		fmt.Printf("Failed to connect to server: %v\n", err)
		return
	}
	defer conn.Close()

	client := pb.NewIPServiceClient(conn)

	stream, err := client.WatchIPUpdates(context.Background(), &pb.EmptyRequest{})
	if err != nil {
		fmt.Printf("Failed to open IP updates stream: %v\n", err)
		return
	}

	for {
		ipUpdate, err := stream.Recv()
		if err != nil {
			fmt.Printf("Error receiving IP update: %v\n", err)
			return
		}

		ip := ipUpdate.IpAddress
		fmt.Printf("Received IP update from server: %s\n", ip)

		latestIPLock.Lock()
		if !isIPInfoExists(ip) {
			ipInfos = append(ipInfos, IPInfo{
				IP:   ip,
				Port: "8080", // Default port
			})
			writeIPInfosToFile()
		}
		latestIPLock.Unlock()
	}
}

func isIPInfoExists(ip string) bool {
	for _, info := range ipInfos {
		if info.IP == ip {
			return true
		}
	}
	return false
}

func writeIPInfosToFile() {
	fileName := "../../Policy/output.json"
	file, err := os.Create(fileName)
	if err != nil {
		fmt.Printf("Failed to create file: %v\n", err)
		return
	}
	defer file.Close()

	enc := json.NewEncoder(file)
	enc.SetIndent("", "  ") // Set indentation for pretty formatting

	if err := enc.Encode(ipInfos); err != nil {
		fmt.Printf("Failed to encode JSON: %v\n", err)
	}
}

func main() {
	go watchForIPUpdates()

	select {} // Keep the main goroutine running
}
*/
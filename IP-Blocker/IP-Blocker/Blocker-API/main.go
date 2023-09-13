package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"sync"
	"time"

	pb "Blocker_API/proto/proto" // Adjust the import path
	"google.golang.org/grpc"
)
var (
	currentIP     string
	currentIPLock = &sync.Mutex{}
	ipChans       = make(map[string]chan string)
	ipChansLock   = &sync.Mutex{}
)

// ... Rest of the code

func main() {
	// Run the server executable in the background
	//go runExecutable("./server/server")

	// Wait for a brief moment to allow the server to start
	time.Sleep(2 * time.Second)

	// Run the client executable in the background
	//go runExecutable("./client/client")

	// Wait for a brief moment to allow the client to start
	time.Sleep(2 * time.Second)

	http.HandleFunc("/api/setip", setIPHandler)
	fmt.Println("RESTful API server listening on :8080")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		fmt.Printf("Failed to serve HTTP: %v\n", err)
	}
}

func runExecutable(execPath string) {
	cmd := exec.Command(execPath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Start(); err != nil {
		fmt.Printf("Error starting executable: %v\n", err)
		return
	}

	if err := cmd.Wait(); err != nil {
		fmt.Printf("Error waiting for executable to finish: %v\n", err)
	}
}


func setIPHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST requests are allowed", http.StatusMethodNotAllowed)
		return
	}

	var request struct {
		IP string `json:"ip"`
	}

	err := json.NewDecoder(r.Body).Decode(&request)
	if err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	ip := request.IP
	if ip != "" {
		setIPOnServer(ip) // Call the SetIP function
		sendIPToGRPC(ip)  // Call the function to send IP to gRPC server
		fmt.Fprintf(w, "IP address set successfully: %s", ip)
	} else {
		http.Error(w, "IP address is required", http.StatusBadRequest)
	}
}

func setIPOnServer(ip string) {
	currentIPLock.Lock()
	defer currentIPLock.Unlock()

	currentIP = ip

	ipChansLock.Lock()
	for _, ch := range ipChans {
		select {
		case ch <- ip:
		default:
		}
	}
	ipChansLock.Unlock()
}

func sendIPToGRPC(ip string) {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		fmt.Printf("Failed to connect to server: %v\n", err)
		return
	}
	defer conn.Close()

	client := pb.NewIPServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	_, err = client.SetIP(ctx, &pb.IPRequest{IpAddress: ip})
	if err != nil {
		fmt.Printf("SetIP on server failed: %v\n", err)
	}
}




/*
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"
	"os"
	"os/exec"

	pb "Blocker_API/proto/proto" // Adjust the import path
	"google.golang.org/grpc"
)

var (
	currentIP     string
	currentIPLock = &sync.Mutex{}
	ipChans       = make(map[string]chan string)
	ipChansLock   = &sync.Mutex{}
)

func setIPHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST requests are allowed", http.StatusMethodNotAllowed)
		return
	}

	var request struct {
		IP string `json:"ip"`
	}

	err := json.NewDecoder(r.Body).Decode(&request)
	if err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	ip := request.IP
	if ip != "" {
		setIPOnServer(ip) // Call the SetIP function
		sendIPToGRPC(ip)  // Call the function to send IP to gRPC server
		fmt.Fprintf(w, "IP address set successfully: %s", ip)
	} else {
		http.Error(w, "IP address is required", http.StatusBadRequest)
	}
}

func setIPOnServer(ip string) {
	currentIPLock.Lock()
	defer currentIPLock.Unlock()

	currentIP = ip

	ipChansLock.Lock()
	for _, ch := range ipChans {
		select {
		case ch <- ip:
		default:
		}
	}
	ipChansLock.Unlock()
}

func sendIPToGRPC(ip string) {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		fmt.Printf("Failed to connect to server: %v\n", err)
		return
	}
	defer conn.Close()

	client := pb.NewIPServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	_, err = client.SetIP(ctx, &pb.IPRequest{IpAddress: ip})
	if err != nil {
		fmt.Printf("SetIP on server failed: %v\n", err)
	}
}

func runCommand(command string, args ...string) {
	cmd := exec.Command(command, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Start(); err != nil {
		fmt.Printf("Error starting command: %v\n", err)
		return
	}

	if err := cmd.Wait(); err != nil {
		fmt.Printf("Error waiting for command to finish: %v\n", err)
	}
}


func main() {

       // Run the server.go in the background
	go runCommand("go", "run", "server/server.go")

	// Wait for a brief moment to allow the server to start
	time.Sleep(2 * time.Second)

	// Run the client.go in the background
	go runCommand("go", "run", "client/client.go")

	// Wait for a brief moment to allow the client to start
	time.Sleep(2 * time.Second)

	http.HandleFunc("/api/setip", setIPHandler)
	fmt.Println("RESTful API server listening on :8080")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		fmt.Printf("Failed to serve HTTP: %v\n", err)
	}
}
*/



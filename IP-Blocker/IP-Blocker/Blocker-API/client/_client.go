package main

import (
	"context"
	"fmt"
	//"sync"

	pb "Blocker_API/proto/proto" // Update the import path
	"google.golang.org/grpc"
)

func watchForIPUpdates() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		fmt.Printf("Failed to connect to server: %v\n", err)
		return
	}
	defer conn.Close()

	client := pb.NewIPServiceClient(conn)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	stream, err := client.WatchIPUpdates(ctx, &pb.EmptyRequest{})
	if err != nil {
		fmt.Printf("Failed to open IP updates stream: %v\n", err)
		return
	}

	for {
		ipUpdate, err := stream.Recv()
		if err != nil {
			fmt.Printf("Error receiving IP update: %v\n", err)
			break
		}

		ip := ipUpdate.IpAddress
		fmt.Printf("Received IP update from server: %s\n", ip)
	}
}

func main() {
	watchForIPUpdates()
}

/*
package main

import (
	"context"
	"fmt"
	"log"
	"sync"
	//"time"

	"google.golang.org/grpc"
	pb "Blocker_API/proto/proto"
	 // Adjust the import path
)

type IPClient struct {
	client pb.IPServiceClient

	mu      sync.Mutex
	ipChans map[string]chan string
}

func NewIPClient(conn *grpc.ClientConn) *IPClient {
	return &IPClient{
		client:  pb.NewIPServiceClient(conn),
		ipChans: make(map[string]chan string),
	}
}

func (c *IPClient) SubscribeIP() <-chan string {
	c.mu.Lock()
	defer c.mu.Unlock()

	ch := make(chan string)
	c.ipChans[fmt.Sprintf("%p", ch)] = ch

	go func() {
		for {
			ipResponse, err := c.client.GetIP(context.Background(), &pb.EmptyRequest{})
			if err != nil {
				log.Printf("GetIP failed: %v", err)
				continue
			}

			ip := ipResponse.GetIpAddress()

			for _, ch := range c.ipChans {
				select {
				case ch <- ip:
				default:
				}
			}
		}
	}()

	return ch
}

func main() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	ipClient := NewIPClient(conn)
	ipChan := ipClient.SubscribeIP()

	for ip := range ipChan {
		fmt.Printf("Received IP address from server: %s\n", ip)
	}
}
*/



/*
package main

import (
	"context"
	"fmt"
	"log"

	"google.golang.org/grpc"

	pb "Blocker_API/proto/proto"  // Adjust the import path
)

func main() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewIPServiceClient(conn)

	ctx := context.Background()

	response, err := client.GetIP(ctx, &pb.EmptyRequest{})
	if err != nil {
		log.Fatalf("GetIP failed: %v", err)
	}

	fmt.Printf("Received IP address from server: %s\n", response.GetIpAddress())
}

*/
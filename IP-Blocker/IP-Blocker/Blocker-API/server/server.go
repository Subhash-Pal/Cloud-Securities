

package main

import (
	"context"
	"fmt"
	"net"
	"sync"
	"time"

	pb "Blocker_API/proto/proto" // Update the import path
	"google.golang.org/grpc"
)

var (
	currentIP     string
	currentIPLock = &sync.Mutex{}
	ipChans       = make(map[string]chan string)
	ipChansLock   = &sync.Mutex{}
)

type server struct {
	pb.UnimplementedIPServiceServer
}

func (s *server) SetIP(ctx context.Context, req *pb.IPRequest) (*pb.EmptyResponse, error) {
	ip := req.IpAddress
	fmt.Printf("Received IP request: %s\n", ip)

	setIPOnServer(ip) // Call the SetIP function

	fmt.Println("IP request processed successfully")
	return &pb.EmptyResponse{}, nil
}

func (s *server) WatchIPUpdates(req *pb.EmptyRequest, stream pb.IPService_WatchIPUpdatesServer) error {
	for {
		ip := getCurrentIP()

		update := &pb.IPUpdate{IpAddress: ip}
		if err := stream.Send(update); err != nil {
			return err
		}

		time.Sleep(time.Second*3) // Adjust the interval as needed
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

func getCurrentIP() string {
	currentIPLock.Lock()
	defer currentIPLock.Unlock()

	return currentIP
}

func main() {
	listener, err := net.Listen("tcp", ":50051")
	if err != nil {
		fmt.Printf("Failed to listen: %v\n", err)
		return
	}

	s := grpc.NewServer()
	pb.RegisterIPServiceServer(s, &server{})

	fmt.Println("gRPC server listening on :50051")
	if err := s.Serve(listener); err != nil {
		fmt.Printf("Failed to serve: %v\n", err)
	}
}




/*package main

import (
	"context"
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"

	pb "Blocker_API/proto/proto" // Adjust the import path

	// Add any other necessary imports
)

type server struct {
	pb.UnimplementedIPServiceServer
}

func (s *server) SendIP(ctx context.Context, ipReq *pb.IPRequest) (*pb.IPResponse, error) {
	ip := ipReq.GetIpAddress()
	fmt.Printf("Received IP address from gRPC: %s\n", ip)
	return &pb.IPResponse{Message: "Received IP address successfully"}, nil
}

func (s *server) GetIP(ctx context.Context, req *pb.EmptyRequest) (*pb.IPResponse, error) {
	return &pb.IPResponse{Message: "Here's your IP address from server", IpAddress: "192.168.1.1"}, nil
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterIPServiceServer(s, &server{})
	fmt.Println("gRPC server listening on :50051")
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
*/
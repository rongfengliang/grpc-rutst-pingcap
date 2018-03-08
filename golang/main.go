package main

import (
	"context"
	"log"

	pb "github.com/rongfengliang/grpc-rs/golang/service/proto"
	grpc "google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial("rpcserver:50051", grpc.WithInsecure())
	if err != nil {
		println("some wrong")
	}
	client := pb.NewUserLoginClient(conn)
	result, err := client.Login(context.Background(), &pb.UserRequest{
		Name: "dalongdemo",
		Age:  "333",
	})
	if err != nil {
		log.Fatal("some wrong")
	}
	log.Println(result.Message)
}

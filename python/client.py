from __future__ import print_function
import grpc
import user_pb2
import user_pb2_grpc
def run():
    channel=grpc.insecure_channel("127.0.0.1:50051")
    stub=user_pb2_grpc.UserLoginStub(channel)
    stub.Login()
    response =stub.Login(user_pb2.UserRequest(name="dalong",age="3333"))
    print(response.message)
if __name__=='__main__':
    run()
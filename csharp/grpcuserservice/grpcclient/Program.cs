using System;
using Grpc.Core;
using userservice;
namespace grpcclient
{
    class MainClass
    {
        public static void Main(string[] args)
        {
            Channel channel = new Channel("127.0.0.1:50052", ChannelCredentials.Insecure);

            var client = new userservice.UserLogin.UserLoginClient(channel);
            String user = "dalong";
            var reply = client.Login(new UserRequest { Name = user, Age = "3eee" });
            Console.WriteLine("hello from : " + reply.Message);
            channel.ShutdownAsync().Wait();
            Console.WriteLine("Press any key to exit...");
            Console.ReadKey();
        }
    }
}

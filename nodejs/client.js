const grpc = require("grpc");
const user_grpc = require("./proto/user_grpc_pb");
const user = require("./proto/user_pb");

function main() {
    var client = new user_grpc.UserLoginClient('47.52.58.151:50051',
        grpc.credentials.createInsecure());
    var request = new user.UserRequest();
    request.setName("dalongdemo");
    request.setAge("demoapp");
    client.login(request, function (err, response) {
        console.log(response);
        console.log('demo info:', response.getMessage());
    });
}

main();
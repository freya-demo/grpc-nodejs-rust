import appRootPath from 'app-root-path';
import { loadSync } from '@grpc/proto-loader';
import * as dotenv from 'dotenv';
import {
  GrpcObject,
  ServiceClientConstructor,
  credentials,
  loadPackageDefinition,
} from '@grpc/grpc-js';
dotenv.config();

export const sum = (a: number, b: number): number => {
  return a + b;
};

const HELLO_WORLD_PROTO_PATH = appRootPath.resolve('protos/hello_world.proto');
const SERVER_ADDR = 'localhost:10000';

async function main() {
  const definition = loadSync(HELLO_WORLD_PROTO_PATH);
  const hello_proto = loadPackageDefinition(definition);
  const hello_package = hello_proto.hello_world as GrpcObject;
  const hello_client =
    new (hello_package.HelloWorld as ServiceClientConstructor)(
      SERVER_ADDR,
      credentials.createInsecure(),
    );
  // console.log(hello_client.helloWorld);
  
  hello_client.helloWorld();

  
}

main();

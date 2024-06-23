import appRootPath from 'app-root-path';
import { loadSync } from '@grpc/proto-loader';
import * as dotenv from 'dotenv';
import {
  GrpcObject,
  ServiceClientConstructor,
  credentials,
  loadPackageDefinition,
} from '@grpc/grpc-js';
import { createChannel, createClient } from 'nice-grpc';
import {
  HelloWorldClient,
  HelloWorldDefinition,
} from '../compiled_protos/hello_world.js';
dotenv.config();

export const sum = (a: number, b: number): number => {
  return a + b;
};

const SERVER_ADDR = 'localhost:10000';

async function main() {
  const channel = createChannel(SERVER_ADDR);
  const client: HelloWorldClient = createClient(HelloWorldDefinition, channel);
  const resp = await client.helloWorld({ helloString: 'hi' });
  console.log(resp);
  const resp2 = await client.echoList({ demoStr: ['a', 'b', 'c'] });
  console.log(resp2);
}

main();

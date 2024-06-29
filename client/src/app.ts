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
import {
  SingleCounterClient,
  SingleCounterDefinition,
} from '../compiled_protos/counter.js';
dotenv.config();

export const sum = (a: number, b: number): number => {
  return a + b;
};

const SERVER_ADDR = 'localhost:10000';

async function main() {
  const channel = createChannel(SERVER_ADDR);

  {
    const helloWorldClient: HelloWorldClient = createClient(
      HelloWorldDefinition,
      channel,
    );
    const [resp1, resp2] = await Promise.all([
      helloWorldClient.helloWorld({ helloString: 'hi' }),
      helloWorldClient.echoList({ demoStr: ['a'] }),
    ]);
    console.log(resp1, resp2);
  }

  {
    const counterClient: SingleCounterClient = createClient(
      SingleCounterDefinition,
      channel,
    );
    const resp1 = await counterClient.increase({ delta: 10 });
    const resp2 = await counterClient.increase({ delta: -2 });
    console.log(resp1, resp2);
  }
}

main();

import { spawn } from 'child_process';
import appRootPath from 'app-root-path';

function main() {
  const CLIENT_DIR = appRootPath.resolve('client');

  // TODO: Add unix script
  // TODO: Add loop
  spawn(
    '"./node_modules/grpc-tools/bin/protoc.exe"',
    [
      `--plugin=protoc-gen-ts_proto=".\\node_modules\\.bin\\protoc-gen-ts_proto.cmd"`,
      `--ts_proto_out=".\\compiled_protos"`,
      `--ts_proto_opt="outputServices=nice-grpc,outputServices=generic-definitions,useExactTypes=false,esModuleInterop=true,importSuffix=.js"`,
      `--proto_path="..\\protos"`,
      `..\\protos\\*.proto`,
    ],
    {
      stdio: 'inherit',
      shell: true,
      cwd: CLIENT_DIR,
    },
  );
}

console.info("----\x1b[32mSCRIPT START\x1b[0m----")

main();

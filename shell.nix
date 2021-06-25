let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      rev = "57c8084c7ef41366993909c20491e359bbb90f54";
    });
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust-nightly = with nixpkgs; ((rustChannelOf { date = "2021-05-20"; channel = "nightly"; }).rust.override { });
  buildAllTools = nixpkgs.writeShellScriptBin "build-all-tools" ''
    build-oasis-go-tool oasis-net-runner
    build-oasis-go-tool oasis-node
    build-oasis-rust-tool runtime-loader
    build-oasis-test-client
  '';
  buildOasisGoTool = nixpkgs.writeShellScriptBin "build-oasis-go-tool" ''
    rm -rf /tmp/$1
    cd oasis-core/go/$1
    go build
    mv $1 /tmp
    ls -l /tmp/$1
    cd ../../
  '';
  buildOasisRustTool = nixpkgs.writeShellScriptBin "build-oasis-rust-tool" ''
    rm -rf /tmp/oasis-core-$1
    cd oasis-core/$1
    cargo build
    cd ../
    mv target/debug/oasis-core-$1 /tmp
    ls -l /tmp/oasis-core-$1
    cd ../
  '';
  buildOasisTestClient = nixpkgs.writeShellScriptBin "build-oasis-test-client" ''
    rm -rf /tmp/minimal-runtime-client
    cd tests
    go build
    mv minimal-runtime-client /tmp
    ls -l /tmp/minimal-runtime-client
    cd ../
  '';
  runTestChain = nixpkgs.writeShellScriptBin "run-test-chain" ''
    rm -rf /tmp/rt-test
    mkdir /tmp/rt-test
    cargo build
    /tmp/oasis-net-runner \
    --fixture.default.node.binary /tmp/oasis-node \
    --fixture.default.runtime.binary target/debug/wasmedge-paratime \
    --fixture.default.runtime.loader /tmp/oasis-core-runtime-loader \
    --fixture.default.runtime.provisioner unconfined \
    --fixture.default.keymanager.binary \'\' \
    --basedir /tmp/rt-test \
    --basedir.no_temp_dir
  '';
in
with nixpkgs; pkgs.mkShell {
  buildInputs = [
    cmake
    libseccomp
    pkgconfig
    rust-nightly
    protobuf
    openssl
    rust-nightly
    go

    buildAllTools
    buildOasisGoTool
    buildOasisRustTool
    buildOasisTestClient
    runTestChain
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
}

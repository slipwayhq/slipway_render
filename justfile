default:
  just --list
  
build target="debug": (build-src target) (build-wasm target)

test *FLAGS: build
  cd src && cargo nextest run {{FLAGS}}

clean: clean-src clean-wasm

build-src target="debug":
  cd src && cargo build {{ if target == "release" { "--release" } else { "" } }}

build-wasm target="debug": && (assemble-component target)
  cd wasm && cargo build {{ if target == "release" { "--release" } else { "" } }}

clean-src:
  cd src && cargo clean

clean-wasm:
  cd wasm && cargo clean

assemble-component target:
  mkdir -p artifacts
  rm -rf artifacts/{{target}}
  mkdir -p artifacts/{{target}}/slipway_renderer
  cp wasm/target/wasm32-wasi/{{target}}/slipway_renderer_component.wasm artifacts/{{target}}/slipway_renderer/slipway_component.wasm
  cp wasm/slipway-renderer-component/slipway_component.json artifacts/{{target}}/slipway_renderer/slipway_component.json
  cp adaptive-cards-data/schema/adaptive-card.schema.json artifacts/{{target}}/slipway_renderer/adaptive-card.schema.json
  cp adaptive-cards-data/schema/host-config-with-defaults.schema.json artifacts/{{target}}/slipway_renderer/host-config-with-defaults.schema.json

  tar -cf artifacts/{{target}}/slipway_renderer.tar -C artifacts/{{target}}/slipway_renderer .

  # Rename the tarball with a name that includes the publisher, name and version.
  publisher=$(jq -r '.publisher' wasm/slipway-renderer-component/slipway_component.json) && \
    name=$(jq -r '.name' wasm/slipway-renderer-component/slipway_component.json) && \
    version=$(jq -r '.version' wasm/slipway-renderer-component/slipway_component.json) && \
    new_filename="${publisher}.${name}.${version}.tar" && \
    mv artifacts/{{target}}/slipway_renderer.tar "artifacts/{{target}}/$new_filename"



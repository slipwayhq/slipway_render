default:
  just --list
  
build configuration="debug": download-fonts (build-src configuration) (build-components configuration)

test *FLAGS: build
  cd src && cargo nextest run {{FLAGS}}
  cd src_components && cargo nextest run {{FLAGS}}

test-snapshot name *FLAGS: build-src
  cd src && SNAPSHOT_TEST_NAME={{name}} cargo nextest run snapshots {{FLAGS}}

debug-snapshot name *FLAGS: build-src
  cd src && SNAPSHOT_TEST_NAME={{name}} SNAPSHOT_DEBUG_MODE=full cargo nextest run snapshots {{FLAGS}}

clean: clean-src clean-components (clean-artifacts "")

build-src configuration="debug":
  cd src && cargo build {{ if configuration == "release" { "--release" } else { "" } }}

build-components configuration="debug": && (assemble-components configuration)
  slipway wit > src_components/slipway_render_component/wit/slipway.wit
  cd src_components && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

clean-src:
  cd src && cargo clean

clean-components:
  cd src_components && cargo clean

assemble-components configuration: \
  (clean-artifacts configuration) \
  (copy-component-files configuration "render") \
  (copy-render-component-additional-files configuration) \
  (tar-component-files configuration "render") \
  (assemble-simple-component configuration "echarts_demo") \

assemble-simple-component configuration name: \
  (copy-all-component-files configuration name) \
  (tar-component-files configuration name) \

clean-artifacts configuration:
  mkdir -p artifacts
  rm -rf artifacts

copy-render-component-additional-files configuration:
  cp adaptive_cards_data/schema/adaptive-card.schema.json artifacts/slipway.render/adaptive-card.schema.json
  cp adaptive_cards_data/schema/host-config-with-defaults.schema.json artifacts/slipway.render/host-config-with-defaults.schema.json

copy-all-component-files configuration name:
  mkdir -p artifacts/slipway.{{name}}
  cp src_components/slipway_{{name}}_component/* artifacts/slipway.{{name}}

copy-component-files configuration name:
  mkdir -p artifacts/slipway.{{name}}
  cp src_components/target/wasm32-wasip2/{{configuration}}/slipway_{{name}}_component.wasm artifacts/slipway.{{name}}/slipway_component.wasm
  cp src_components/slipway_{{name}}_component/slipway_component.json artifacts/slipway.{{name}}/slipway_component.json

tar-component-files configuration name:
  slipway package artifacts/slipway.{{name}}

download-fonts:
  ./download_fonts.sh

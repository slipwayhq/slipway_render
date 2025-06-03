publisher := "slipwayhq"

default:
  just --list

build configuration="release": download-fonts (build-src configuration) wit (build-components configuration)
build-ci: && (build-src "release") (build-components "release")
  cargo install cargo-typify
  rustup target add wasm32-wasip2

test *FLAGS: (build "release")
  cd src && cargo nextest run --release {{FLAGS}}
  cd src_components && cargo nextest run --release {{FLAGS}}

test-snapshot name *FLAGS: build-src
  cd src && SNAPSHOT_TEST_NAME={{name}} cargo nextest run snapshots {{FLAGS}}

debug-snapshot name *FLAGS: build-src
  cd src && SNAPSHOT_TEST_NAME={{name}} SNAPSHOT_DEBUG_MODE=full cargo nextest run snapshots {{FLAGS}}

clean: clean-src clean-components (clean-component-artifacts "")

build-src configuration="release":
  cd src && cargo build {{ if configuration == "release" { "--release" } else { "" } }}

build-components configuration="release": && (assemble-components configuration)
  cd src_components && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

wit:
  slipway wit > src_components/slipway_render_component/wit/slipway.wit

clean-src:
  cd src && cargo clean

clean-components:
  cd src_components && cargo clean

assemble-components configuration: \
  (clean-component-artifacts configuration) \
  (copy-component-files configuration "render") \
  (copy-render-component-additional-files configuration) \
  (tar-component-files configuration "render") \
  (assemble-simple-component configuration "echarts_demo") \

assemble-simple-component configuration name: \
  (copy-all-component-files configuration name) \
  (tar-component-files configuration name) \

clean-component-artifacts configuration:
  mkdir -p components
  rm -rf components

copy-render-component-additional-files configuration:
  cp adaptive_cards_data/schema/host-config-with-defaults.schema.json components/{{publisher}}.render/host-config-with-defaults.schema.json

copy-all-component-files configuration name:
  mkdir -p components/{{publisher}}.{{name}}
  cp src_components/slipway_{{name}}_component/* components/{{publisher}}.{{name}}

copy-component-files configuration name:
  mkdir -p components/{{publisher}}.{{name}}
  cp src_components/target/wasm32-wasip2/{{configuration}}/slipway_{{name}}_component.wasm components/{{publisher}}.{{name}}/run.wasm
  cp src_components/slipway_{{name}}_component/slipway_component.json components/{{publisher}}.{{name}}/slipway_component.json

tar-component-files configuration name:
  docker run --rm -v "$(pwd)/components":/workspace -w /workspace slipwayhq/slipway:latest slipway package {{publisher}}.{{name}}

download-fonts:
  ./download_fonts.sh

release version: test
  git tag -a "{{version}}" -m "Release {{version}}"
  git push origin "{{version}}"

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
  slipway wit > src_components/slipway_modify_component/wit/world.wit
  slipway wit > src_components/slipway_render_component/wit/world.wit
  slipway wit > src_components/slipway_theme_component/wit/world.wit
  slipway wit > src_components/slipway_svg_component/wit/world.wit
  cd src_components && cargo component build {{ if configuration == "release" { "--release" } else { "" } }}

clean-src:
  cd src && cargo clean

clean-components:
  cd src_components && cargo clean

assemble-components configuration: \
  (clean-artifacts configuration) \
  (copy-component-files configuration "render") \
  (copy-render-component-additional-files configuration) \
  (tar-component-files configuration "render") \
  (rename-component-tar configuration "render") \
  (copy-component-files configuration "svg") \
  (tar-component-files configuration "svg") \
  (rename-component-tar configuration "svg") \
  (copy-component-files configuration "modify") \
  (tar-component-files configuration "modify") \
  (rename-component-tar configuration "modify") \
  (copy-component-files configuration "theme") \
  (copy-theme-component-additional-files configuration) \
  (tar-component-files configuration "theme") \
  (rename-component-tar configuration "theme") \
  (assemble-simple-component configuration "echarts") \
  (assemble-simple-component configuration "echarts_svg") \
  (assemble-simple-component configuration "echarts_demo") \

assemble-simple-component configuration name: \
  (copy-all-component-files configuration name) \
  (tar-component-files configuration name) \
  (rename-component-tar configuration name) \

clean-artifacts configuration:
  mkdir -p artifacts
  rm -rf artifacts

copy-render-component-additional-files configuration:
  cp adaptive_cards_data/schema/adaptive-card.schema.json artifacts/slipway_render/adaptive-card.schema.json
  cp adaptive_cards_data/schema/host-config-with-defaults.schema.json artifacts/slipway_render/host-config-with-defaults.schema.json

copy-theme-component-additional-files configuration:
  cp adaptive_cards_data/schema/host-config-with-defaults.schema.json artifacts/slipway_theme/host-config-with-defaults.schema.json

copy-all-component-files configuration name:
  mkdir -p artifacts/slipway_{{name}}
  cp src_components/slipway_{{name}}_component/* artifacts/slipway_{{name}}

copy-component-files configuration name:
  mkdir -p artifacts/slipway_{{name}}
  cp src_components/target/wasm32-wasip1/{{configuration}}/slipway_{{name}}_component.wasm artifacts/slipway_{{name}}/slipway_component.wasm
  cp src_components/slipway_{{name}}_component/slipway_component.json artifacts/slipway_{{name}}/slipway_component.json

tar-component-files configuration name:
  tar -cf artifacts/slipway_{{name}}.tar -C artifacts/slipway_{{name}} .

rename-component-tar configuration name:
  # Rename the tarball with a name that includes the publisher, name and version.
  publisher=$(jq -r '.publisher' src_components/slipway_{{name}}_component/slipway_component.json) && \
    name=$(jq -r '.name' src_components/slipway_{{name}}_component/slipway_component.json) && \
    version=$(jq -r '.version' src_components/slipway_{{name}}_component/slipway_component.json) && \
    new_filename="${publisher}.${name}.${version}.tar" && \
    mv artifacts/slipway_{{name}}.tar "artifacts/$new_filename"

download-fonts:
  ./download_fonts.sh

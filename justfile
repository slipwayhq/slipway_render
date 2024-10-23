default:
  just --list
  
build configuration="debug": (build-src configuration) (build-components configuration)

test *FLAGS: build
  cd src && cargo nextest run {{FLAGS}}
  cd src-components && cargo nextest run {{FLAGS}}

clean: clean-src clean-components (clean-artifacts "")

build-src configuration="debug":
  cd src && cargo build {{ if configuration == "release" { "--release" } else { "" } }}

build-components configuration="debug": && (assemble-components configuration)
  cd src-components && cargo build --target "wasm32-wasi" {{ if configuration == "release" { "--release" } else { "" } }}

clean-src:
  cd src && cargo clean

clean-components:
  cd src-components && cargo clean

assemble-components configuration: \
  (clean-artifacts configuration) \
  (copy-component-files configuration "render") \
  (copy-render-component-additional-files configuration) \
  (tar-component-files configuration "render") \
  (rename-component-tar configuration "render") \
  (copy-component-files configuration "modify") \
  (tar-component-files configuration "modify") \
  (rename-component-tar configuration "modify") \
  (copy-component-files configuration "theme") \
  (tar-component-files configuration "theme") \
  (rename-component-tar configuration "theme") \

clean-artifacts configuration:
  mkdir -p artifacts
  rm -rf artifacts/{{configuration}}

copy-render-component-additional-files configuration:
  cp adaptive-cards-data/schema/adaptive-card.schema.json artifacts/{{configuration}}/slipway_render/adaptive-card.schema.json
  cp adaptive-cards-data/schema/host-config-with-defaults.schema.json artifacts/{{configuration}}/slipway_render/host-config-with-defaults.schema.json

copy-component-files configuration name:
  mkdir -p artifacts/{{configuration}}/slipway_{{name}}
  cp src-components/target/wasm32-wasi/{{configuration}}/slipway_{{name}}_component.wasm artifacts/{{configuration}}/slipway_{{name}}/slipway_component.wasm
  cp src-components/slipway-{{name}}-component/slipway_component.json artifacts/{{configuration}}/slipway_{{name}}/slipway_component.json

tar-component-files configuration name:
  tar -cf artifacts/{{configuration}}/slipway_{{name}}.tar -C artifacts/{{configuration}}/slipway_{{name}} .


rename-component-tar configuration name:
  # Rename the tarball with a name that includes the publisher, name and version.
  publisher=$(jq -r '.publisher' src-components/slipway-{{name}}-component/slipway_component.json) && \
    name=$(jq -r '.name' src-components/slipway-{{name}}-component/slipway_component.json) && \
    version=$(jq -r '.version' src-components/slipway-{{name}}-component/slipway_component.json) && \
    new_filename="${publisher}.${name}.${version}.tar" && \
    mv artifacts/{{configuration}}/slipway_{{name}}.tar "artifacts/{{configuration}}/$new_filename"

name := 'app-hub'
appid := 'com.AppHub.AppHub'
version := '1.0.0-alpha.2'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))

bin-src := 'target' / 'release' / name
bin-dst := base-dir / 'bin' / name

desktop := appid + '.desktop'
desktop-src := 'res' / desktop
desktop-dst := clean(rootdir / prefix) / 'share' / 'applications' / desktop

icons-src := 'res' / 'icons' / 'hicolor'
icons-dst := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'

icon-svg-src := icons-src / 'scalable' / 'apps' / 'icon.svg'
icon-svg-dst := icons-dst / 'scalable' / 'apps' / appid + '.svg'

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Compiles with debug profile
build-debug *args:
    cargo build {{args}} --all

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Run the application for testing purposes
run *args:
    just build-release {{args}}
    env RUST_LOG=app_hub=info,backend=info,common_utils=info RUST_BACKTRACE=full cargo run --release {{args}}

# Installs files
install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 res/{{name}}.desktop {{desktop-dst}}
    install -Dm0644 {{icon-svg-src}} {{icon-svg-dst}}

# Uninstalls installed files
uninstall:
    rm {{bin-dst}} {{desktop-dst}} {{icon-svg-dst}}

package-deb:
    mkdir -p debian/usr/bin
    mkdir -p debian/usr/share/applications
    mkdir -p debian/usr/share/icons/hicolor/scalable/apps
    mkdir -p debian/DEBIAN

    install -Dm0755 {{bin-src}} debian{{bin-dst}}
    install -Dm0644 res/{{name}}.desktop debian{{desktop-dst}}
    install -Dm0644 {{icon-svg-src}} debian{{icon-svg-dst}}

    echo "Package: {{name}}" > debian/DEBIAN/control
    echo "Version: {{version}}" >> debian/DEBIAN/control
    echo "Section: utils" >> debian/DEBIAN/control
    echo "Priority: optional" >> debian/DEBIAN/control
    echo "Architecture: amd64" >> debian/DEBIAN/control
    echo "Maintainer: Francesco Pio Gaglione <francesco.gaglione.p@gmail.com>" >> debian/DEBIAN/control
    echo "Description: AppHub is a Linux desktop application that simplifies the installation and management of .appImage packages through an intuitive graphical interface. Additionally, it provides the ability to easily uninstall applications installed via AppImage. " >> debian/DEBIAN/control

    chmod 755 debian/DEBIAN
    chmod 644 debian/DEBIAN/control

    dpkg-deb --build debian

    mv debian.deb {{name}}_{{version}}_amd64.deb

    rm -rf debian

# Vendor dependencies locally
vendor:
    #!/usr/bin/env bash
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    echo >> .cargo/config.toml
    echo '[env]' >> .cargo/config.toml
    if [ -n "${SOURCE_DATE_EPOCH}" ]
    then
        source_date="$(date -d "@${SOURCE_DATE_EPOCH}" "+%Y-%m-%d")"
        echo "VERGEN_GIT_COMMIT_DATE = \"${source_date}\"" >> .cargo/config.toml
    fi
    if [ -n "${SOURCE_GIT_HASH}" ]
    then
        echo "VERGEN_GIT_SHA = \"${SOURCE_GIT_HASH}\"" >> .cargo/config.toml
    fi
    tar pcf vendor.tar .cargo vendor
    rm -rf .cargo vendor

# Extracts vendored dependencies
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar

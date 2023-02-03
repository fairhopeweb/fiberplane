#!/usr/bin/env bash

set -eu

function print_usage_and_exit() {
    echo "Usage: scripts/publish_crates.sh --registry=(artifactory|crates-io) [--all]"
    echo ""
    echo "By default, only crates that were changed in the last commit are published."
    echo "Use --all to publish all crates."
    exit 1
}

all="false"
registry=""
for arg in "$@"; do
  case $arg in
    -a|--all)
      all="true"
      shift
      ;;
    -r=*|--registry=*)
      registry="${arg#*=}"
      shift
      ;;
    *)
      echo -e "Unknown option or argument: $arg\n"
      print_usage_and_exit
      ;;
  esac
done

if [[ $registry != "artifactory" && $registry != "crates-io" ]]; then
    echo -e "No or unknown registry specified.\n"
    print_usage_and_exit
fi

# Crate dirs in the order in which they should be published:
CRATES=(
    "base64uuid"
    "fiberplane-models"
    "fiberplane-api-client"
    "fiberplane-markdown"
    "fiberplane-provider-protocol/fiberplane-provider-bindings"
    "fiberplane-provider-protocol/fiberplane-provider-runtime"
    "fiberplane-templates"
    "fiberplane"
)

realpath="$( dirname "$0" )/realpath.sh"

# Make sure we start in the repository root
root="$( dirname "$( dirname "$( $realpath $0 )" )" )"
cd $root

# Determine the latest commits
last_commits=`git log --pretty=%t | head -n 2`
curr_commit=`echo "$last_commits" | head -n 1`
prev_commit=`echo "$last_commits" | tail -n 1`

# Determine path to dasel, or mark it for installation
dasel=`which dasel 2> /dev/null || echo "needs install"`

function publish_crate() {
    local crate_dir=$1
    local crate=`echo $crate_dir | sed 's/.*\///'`

    local allow_dirty=""
    local repo_version=`$dasel -f Cargo.toml ".workspace.dependencies.$crate.version"`
    if [[ $registry == "artifactory" ]]; then
        # For Artifactory, we rewrite the versions in Cargo.toml so we can
        # publish every commit under a unique version. We restore Cargo.toml
        # at the end of the script.
        local base_version=`echo $repo_version | sed 's/-.*//'`
        local version="${base_version}-${curr_commit}"
        $dasel put -f Cargo.toml -s ".workspace.dependencies.$crate.version" -v $version
        $dasel put -f Cargo.toml -s ".workspace.dependencies.$crate.registry" -v $registry

        $dasel put -f "$crate_dir/Cargo.toml" -s ".package.version" -v $version

        allow_dirty="--allow-dirty"
    else
        local version=$repo_version
    fi

    # Publish
    echo "Publishing $crate v$version..."
    pushd $crate_dir
    cargo publish --registry=$registry $allow_dirty

    if [[ $registry == "artifactory" ]]; then
        git restore "Cargo.toml"
    fi

    popd
}

function check_dasel_install() {
    if [[ $dasel != "needs install" ]]; then
        return 0
    fi

    local local_bin=~/.local/bin
    dasel="$local_bin/dasel"

    local version=$(curl -s https://api.github.com/repos/tomwright/dasel/releases/latest | jq -r '.tag_name')
    echo "Installing dasel $version to $local_bin..."

    mkdir -p $local_bin
    local platform=`get_platform`
    local arch=`get_architecture`
    curl -L "https://github.com/TomWright/dasel/releases/download/${version}/dasel_${platform}_${arch}" --output $dasel
    chmod a+x $dasel
}

function get_architecture() {
    case $(uname -m) in
        x86_64) echo "amd64" ;;
        arm)    echo "arm64" ;;
        *)      echo "Unrecognized architecture"; exit 1 ;;
    esac
}

function get_platform() {
    case $(uname -s) in
        Darwin) echo "darwin" ;;
        Linux)  echo "linux" ;;
        *)      echo "Unrecognized platform"; exit 1 ;;
    esac
}

# Go through all the crates and see which need publishing
did_publish="false"
for crate_dir in ${CRATES[@]}; do
    has_changed=`git diff --quiet HEAD $prev_commit -- $crate_dir || echo changed`
    if [[ $has_changed != "changed" && $all == "false" ]]; then
        continue
    fi

    check_dasel_install

    publish_crate $crate_dir
    did_publish="true"
done

if [[ $did_publish == "true" ]]; then
    git restore Cargo.toml
else
    echo "No crates needed publishing."
fi

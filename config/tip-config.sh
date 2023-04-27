# Adds tip install folder to PATH if not already added
if [[ ":$PATH:" != *":/home/whiskerz/Projects/tip:"* ]]; then
    export PATH="$PATH:/home/whiskerz/Projects/tip"
fi

# Begin tip configuration
function load_targets() {
    while IFS='=' read -r name address; do
        export "$name=$address"
    done < "/home/whiskerz/Projects/tip/target/release/targets.txt"
}

# Call the load_targets function during shell initialization
load_targets

# Shell function to allow sourcing
function tip() {
  /home/whiskerz/Projects/tip/target/release/tip "$@"
  source /home/whiskerz/Projects/tip/target/release/config/tip-config.sh
}
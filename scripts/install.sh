#!/usr/bin/bash
set -eu
shopt -s nullglob

PROFILE_DIR="profiles"

function info {
	printf ' \e[0;36m[*]\e[0m %s\n' "$@"
}

function warn {
	printf ' \e[0;33m[!]\e[0m %s\n' "$@"
}

function load_profile {
	# auto export all variable
	set -a
	# jq outputs the json structure in a `key=value` format
	eval "$(jq -r 'to_entries[] | "\(.key)=\(.value)"' "${PROFILE_DIR}/${PROFILE}.json")"
	set +a
}

# install_overlay "<overlay src dir>" "<dest dir>"
function install_overlay {
	local src_dir
	local dst_dir

	# ensure these exist
	src_dir="$(realpath -e "$1")"
	dst_dir="$(realpath -e "$2")"

	# paths relative to the overlay root
	local rel_paths
	mapfile -t rel_paths < <(find "$src_dir" -not \( -path '*/.hooks*' -or -path '*/.gitkeep' \) -printf '%P\n') # this find command probably wont work on anything other than gnuutils
	# exlcude all:
	#  .gitkeep files
	#  the .hooks directory

	# absolute paths to the source overlay files
	local src_paths=()
	for p in "${rel_paths[@]}"; do
		src_paths+=("$(realpath -e "${src_dir}/$p")")
	done

	# absolute paths to the destination files
	local dst_paths=()
	for p in "${rel_paths[@]}"; do
		dst_paths+=("$(realpath -m "${dst_dir}/$(eval "echo $p")")")
	done

	for ((i = 0; i < "${#rel_paths[@]}"; i++)); do
		local src="${src_paths[i]}"
		local dst="${dst_paths[i]}"

		if [ -d "$src" ]; then
			info "D: $dst"
			mkdir -p -- "$dst"
		elif [ -r "$src" ]; then
			info "F: $src -> $dst"
			install -Dm644 -- "$src" "$dst"
		fi
	done

	# Hooks
	for hook in "$src_dir"/.hooks/*; do
		if [[ -x "$hook" ]]; then
			info "Running hook: $hook"
			"$hook"
		else
			warn "Skipping hook: $hook - not executable"
		fi
	done
}

load_profile

destdir="$(realpath -e "$DESTDIR")"
export destdir
export DESTDIR

# Main overlay
info "Installing main overlay"
info "======================="
install_overlay "./overlay/" "$destdir"

printf '\n'
info "Installing runtime overlays"
info "==========================="

# Runtime overlays
for rt_dir in runtimes/*/; do
	rt_overlay_path="$rt_dir/overlay"
	if [ -d "$rt_overlay_path" ]; then
		info "Installing runtime for: $rt_dir"
		install_overlay "$rt_overlay_path" "$destdir"
	fi
done

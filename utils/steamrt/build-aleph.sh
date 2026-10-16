podman run \
 --rm \
 --init \
 -v $(pwd):/aleph:ro,z \
 -v $(pwd)/.aleph/steamrt:/out:rw,z \
 -h "$(hostname)" \
 -it \
 aleph-steamrt \
 /root/utils/steamrt/build-in-container.sh
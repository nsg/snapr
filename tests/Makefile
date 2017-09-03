
test: container push patch
	lxc exec snapr-test /usr/bin/snapr

patch:
	lxc exec snapr-test -- \
		bash -c 'echo SNAPPY_FORCE_API_URL="http://localhost:8000" >> /etc/environment'
	lxc exec snapr-test -- \
		bash -c 'echo SNAPPY_FORCE_CPI_URL="http://localhost:8000" >> /etc/environment'
	lxc exec snapr-test -- apt update
	lxc exec snapr-test -- apt install -y squashfuse
	#lxc exec snapr-test -- snap install core # remove this when we have proxy support
	lxc exec snapr-test -- systemctl restart snapd

push: build dummy_0.1_amd64.snap
	lxc file push ../target/debug/snapr snapr-test/usr/bin/snapr
	lxc exec snapr-test -- mkdir -p /var/snapr

	# Our home grown snap that's not signed
	lxc file push dummy_0.1_amd64.snap snapr-test/var/snapr/dummy_0.1_amd64.snap

	# A signed snap from the store
	lxc exec snapr-test -- wget -O /var/snapr/hello-world.snap https://api.snapcraft.io/api/v1/snaps/download/buPKUD3TKqCOgLEjjHx5kSiCpIs5cMuQ_27.snap

build:
	cd .. && cargo build

container:
	lxc list snapr-test | grep -q snapr-test || lxc launch ubuntu:16.04 snapr-test
	while ! lxc list -c 4 snapr-test | grep -q eth; do sleep 1; done

dummy_0.1_amd64.snap:
	snapcraft

clean:
	lxc delete -f snapr-test || :
	snapcraft clean
	rm -f *.snap

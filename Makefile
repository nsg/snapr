
test: container push patch
	lxc exec snapr-test /usr/bin/snapr

patch:
	lxc exec snapr-test -- \
		bash -c 'echo SNAPPY_FORCE_API_URL="http://localhost:8000" >> /etc/environment'
	lxc exec snapr-test -- \
		bash -c 'echo SNAPPY_FORCE_CPI_URL="http://localhost:8000" >> /etc/environment'
	lxc exec snapr-test -- systemctl restart snapd

push: build
	lxc file push target/debug/snapr snapr-test/usr/bin/snapr

build:
	cargo build

container:
	lxc list snapr-test | grep -q snapr-test || lxc launch ubuntu:16.04 snapr-test
	while ! lxc list -c 4 snapr-test | grep -q eth; do sleep 1; done

clean:
	lxc delete -f snapr-test

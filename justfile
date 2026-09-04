run *args:
	cargo run {{args}}

format:
	cargo +nightly fmt

lint:
	cargo clippy --all

install:
	cargo install --path .
	install -Dm644 systemd/proqbit.service ~/.config/systemd/user/proqbit.service
	systemctl --user daemon-reload
	systemctl --user enable proqbit.service
	systemctl --user restart proqbit.service

uninstall:
	systemctl --user stop proqbit.service || true
	systemctl --user disable proqbit.service || true
	rm -f ~/.config/systemd/user/proqbit.service
	systemctl --user daemon-reload
	systemctl --user reset-failed proqbit.service || true
	cargo uninstall proqbit || true

logs:
	journalctl --user -u proqbit.service -f

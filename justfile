# Launch a development server.
dev:
	nix-shell --command ./dev.py

# Check slide timing information
timing:
	nix-shell timing.nix --command ./timing.py

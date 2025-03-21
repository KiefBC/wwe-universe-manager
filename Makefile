.PHONY: test test-all test-frontend test-backend

# Default target - run all tests
test: test-all

# Run tests for all packages in the workspace
test-all:
	@echo "Running tests for all packages in the workspace..."
	@cargo test --workspace

# Run tests for the frontend package only
test-frontend:
	@echo "Running tests for the frontend package..."
	@cargo test -p wwe-universe-manager-frontend

# Run tests for the backend package only
test-backend:
	@echo "Running tests for the backend package..."
	@cargo test -p wwe-universe-manager

# Clean the project
clean:
	@echo "Cleaning the project..."
	@cargo clean

# Help target
help:
	@echo "Available targets:"
	@echo "  test         - Run all tests (same as test-all)"
	@echo "  test-all     - Run tests for all packages"
	@echo "  test-frontend - Run tests for the frontend package only"
	@echo "  test-backend - Run tests for the backend package only"
	@echo "  clean        - Clean the project"
	@echo "  help         - Show this help message" 
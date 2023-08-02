# Use an official Rust image as a parent image
FROM rustlang/rust:nightly

# Set the working directory
WORKDIR /app

# Create working directories required by Noir
RUN mkdir -p /app/tmp/noir_projects

# Install noirup for Noir language and ensure nargo is installed
RUN curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash - && /bin/bash -c "source $HOME/.bashrc && noirup && noirup --version"

# Add nargo directory to the PATH
ENV PATH="/root/.nargo/bin:${PATH}"

# Copy the current directory (our Rust code) into the Docker image
COPY . .

# Build your rust application
RUN cargo build --release 

# Expose port for the application 
EXPOSE 8080

# Specify the command to run the application
CMD ["./target/release/server"]

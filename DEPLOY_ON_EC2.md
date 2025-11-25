Deployment Guide: Rust Aquarium on AWS EC2This guide assumes you have an AWS account and a basic understanding of the command line.Step 1: Launch an EC2 InstanceLog in to the AWS Console and navigate to EC2.Click Launch Instance.Name: RustAquariumAMI: Amazon Linux 2023 (or Ubuntu 22.04).Instance Type: t2.micro (Free tier eligible).Key Pair: Create one or select an existing one (so you can SSH in).Network Settings: Ensure "Allow SSH traffic from" is checked (set to My IP for security).Launch the instance.Step 2: Connect to your InstanceOpen your terminal and SSH into the instance using your key pair:chmod 400 your-key.pem
ssh -i "your-key.pem" ec2-user@your-instance-public-ip
# Note: If using Ubuntu AMI, use ubuntu@... instead of ec2-user@...
Step 3: Install Docker on EC2Run the following commands inside your EC2 instance:# Update installed packages
sudo dnf update -y  # use 'apt update' if on Ubuntu

# Install Docker
sudo dnf install docker -y # use 'apt install docker.io' if on Ubuntu

# Start Docker service
sudo service docker start

# Add user to docker group (avoids using sudo for every docker command)
sudo usermod -a -G docker ec2-user

# IMPORTANT: You must log out and log back in for group changes to take effect!
exit
Reconnect via SSH (Step 2) before proceeding.Step 4: Create the FilesYou need to get the code onto the server. You can use git or just create the files manually since there are only three.Create the project folder:mkdir rust-aquarium
cd rust-aquarium
mkdir src
Create Cargo.toml:nano Cargo.toml
# Paste the content of the Cargo.toml file provided
# Press Ctrl+O, Enter to save, Ctrl+X to exit
Create src/main.rs:nano src/main.rs
# Paste the content of the src/main.rs file provided
Create Dockerfile:nano Dockerfile
# Paste the content of the Dockerfile provided
Step 5: Build and RunNow, build the Docker image. This might take a few minutes as it compiles the Rust code.docker build -t aquarium .
Once built, run the aquarium.Crucial: You must use the -it flags.-i (interactive): Keeps STDIN open.-t (tty): Allocates a pseudo-TTY, which allows the ASCII graphics to render correctly.docker run -it --rm aquarium
ControlsWatch the fish swim!Press q or Esc to exit the application.

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}


provider "aws" {
  region = "ap-northeast-3"
}

resource "aws_security_group" "instance" {
  name = "simple-web-sg"
  ingress {
    from_port = 8080
    to_port = 8080
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_instance" "test_terraform" {
  ami         = "ami-0a07ff89aacad043e" # Instance OS image ami ID
  instance_type = "t3.micro"
#   vpc_security_group_ids = ["${aws_security_group.instance.id}"]
  vpc_security_group_ids = [aws_security_group.instance.id]

  tags = {
    name = "test_terraform"
  }

  user_data = <<-EOF
    #!/bin/bash
    echo "Hello, Terraform!!" > index.html
    nohup busybox httpd -f -p 8080 &
    EOF
}

resource "aws_vpc" "example" {
  cidr_block = "172.16.5.0/24"

  tags = {
    Name = "test_vpc"
  }
}
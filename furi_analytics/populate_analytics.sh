#!/bin/bash

# Define your Redpanda broker and topic
BROKER="127.0.0.1:9092"
TOPIC="analytics"
N=${1:-100}

# Function to generate a random IP address
generate_ip() {
	echo "$((RANDOM % 256)).$((RANDOM % 256)).$((RANDOM % 256)).$((RANDOM % 256))"
}

# Function to generate a random date in ISO 8601 format
generate_date() {
	date +%s
}

# Function to generate a random URI
generate_uri() {
	uris=("12jkn3a" "heklrta" "qlkerma" "fghnoam" "m1kl23x")
	echo "${uris[$RANDOM % ${#uris[@]}]}"
}

# Function to generate a random User-Agent
generate_user_agent() {
	agents=("Mozilla/5.0 (X11; Linux i686; rv:126.0) Gecko/20100101 Firefox/126.0" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36" "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/25.0 Chrome/121.0.0.0 Mobile Safari/537.3" "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/125.0.6422.80 Mobile/15E148 Safari/604.")
	echo "${agents[$RANDOM % ${#agents[@]}]}"
}

# Function to generate a random country
generate_country() {
	countries=("US" "CA" "GB" "DE" "FR" "JP" "CN" "IN" "BR" "AU")
	echo "${countries[$RANDOM % ${#countries[@]}]}"
}

# Function to generate a JSON object
generate_json() {
	date=$(generate_date)
	uri=$(generate_uri)
	ip=$(generate_ip)
	user_agent=$(generate_user_agent)
	country=$(generate_country)

	echo "{\"date\":\"$date\",\"uri\":\"$uri\",\"ip\":\"$ip\",\"user_agent\":\"$user_agent\",\"country\":\"$country\"}"
}

# Produce messages to the topic
produce_messages() {
	for i in $(seq ${N}); do
		json=$(generate_json)
		echo "$json" | kcat -b $BROKER -t $TOPIC -P
		echo "${i} - Produced message: $json"
	done
}

# Run the produce_messages function
produce_messages

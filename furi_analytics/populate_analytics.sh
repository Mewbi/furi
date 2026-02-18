#!/bin/bash

# Define timescale connection parameters
HOST="localhost"
PORT="5433"
USER="user"
PASSWORD="pass"
DB="analytics"
N=${1:-100}

# Function to generate a random IP address
generate_ip() {
	echo "$((RANDOM % 256)).$((RANDOM % 256)).$((RANDOM % 256)).$((RANDOM % 256))"
}

# Function to generate a random date in ISO 8601 format
generate_date() {
	# Generate a random variation
	d=$(date +%s)
	v=$(($RANDOM % 3600))
	echo $((d - v))
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

# Function to generate a sql statement for inserting a record into the analytics table 
generate_sql() {
	date=$(generate_date)
	uri=$(generate_uri)
	ip=$(generate_ip)
	user_agent=$(generate_user_agent)
	country=$(generate_country)

	echo "INSERT INTO analytics_raw (time, uri, ip, user_agent, country) VALUES (to_timestamp($date), '$uri', '$ip', '$user_agent', '$country');"
}

populate_database() {
	for i in $(seq ${N}); do
		sql=$(generate_sql)
		PGPASSWORD=$PASSWORD psql -h $HOST -p $PORT -U $USER -d $DB -c "$sql"
		echo "${i} - Inserted message: $sql"
	done
}

# Run the populate_database function
populate_database
package com.pxc.spring_kafka_con;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableAsync;

@EnableAsync
@SpringBootApplication
public class SpringKafkaConApplication {

	public static void main(String[] args) {
		SpringApplication.run(SpringKafkaConApplication.class, args);
	}

}

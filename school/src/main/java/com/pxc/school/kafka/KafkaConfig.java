package com.pxc.school.kafka;

import org.apache.kafka.clients.admin.NewTopic;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.kafka.config.TopicBuilder;

@Configuration
public class KafkaConfig {
    @Bean
    public NewTopic itemTopic(){
        return TopicBuilder
                .name("axum1-topic")
                .build();
    }

    @Bean
    public NewTopic unitTopic(){
        return TopicBuilder
                .name("axum2-topic")
                .build();
    }
}

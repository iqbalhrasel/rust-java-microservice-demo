package com.pxc.spring_kafka_con.kafka;

import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.messaging.MessagingException;
import org.springframework.stereotype.Service;

@Service
public class KafkaConsumer {

    @KafkaListener(topics = "kafka-topic-spring")
    public void kafkaListener(DataDemoDto dataDemoDto) throws MessagingException {
        System.out.println("-->> Data received <<--: " + dataDemoDto);
    }
}

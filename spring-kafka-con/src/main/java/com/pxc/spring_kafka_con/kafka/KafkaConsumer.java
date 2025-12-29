package com.pxc.spring_kafka_con.kafka;

import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.messaging.MessagingException;
import org.springframework.stereotype.Service;

@Service
public class KafkaConsumer {

    @KafkaListener(topics = "kafka1-topic-spring", groupId = "kafka1SpringGroup")
    public void kafkaListener(DataDemoDto dataDemoDto) throws MessagingException {
        System.out.println("-->> Data received <<--: " + dataDemoDto);
    }

    @KafkaListener(topics = "kafka2-topic-spring", groupId = "kafka2SpringGroup")
    public void kafkaListener2(ElementDemoDto elementDemoDto) throws MessagingException {
        System.out.println("-->> Element received <<--: " + elementDemoDto);
    }
}

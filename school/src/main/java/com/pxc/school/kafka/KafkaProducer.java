package com.pxc.school.kafka;

import lombok.AllArgsConstructor;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.kafka.support.KafkaHeaders;
import org.springframework.messaging.Message;
import org.springframework.messaging.support.MessageBuilder;
import org.springframework.stereotype.Service;

@AllArgsConstructor
@Service
public class KafkaProducer {
    private final KafkaTemplate<String, ItemDemoDto> template;

    public void sendKafkaMessage1(ItemDemoDto itemDemoDto){
        System.out.println("-->> Sending itemDemoDto <<--");

        Message<ItemDemoDto> message = MessageBuilder
                .withPayload(itemDemoDto)
                .setHeader(KafkaHeaders.TOPIC, "axum1-topic")
                .build();

        template.send(message);
    }

    public void sendKafkaMessage2(UnitDemoDto unitDemoDto){
        System.out.println("-->> Sending unitDemoDto <<--");

        Message<UnitDemoDto> message = MessageBuilder
                .withPayload(unitDemoDto)
                .setHeader(KafkaHeaders.TOPIC, "axum2-topic")
                .build();

        template.send(message);
    }
}

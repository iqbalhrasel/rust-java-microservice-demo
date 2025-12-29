package com.pxc.school.kafka;

import lombok.AllArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@AllArgsConstructor
@RestController
@RequestMapping("/kafka")
public class KafkaController {
    private final KafkaProducer kafkaProducer;

    @PostMapping("/produce1")
    public ResponseEntity<?> sendKafkaMessage1(@RequestBody ItemDemoDto dto){
        kafkaProducer.sendKafkaMessage1(dto);
        return ResponseEntity.ok("kafka message sent");
    }

    @PostMapping("/produce2")
    public ResponseEntity<?> sendKafkaMessage2(@RequestBody UnitDemoDto dto){
        kafkaProducer.sendKafkaMessage2(dto);
        return ResponseEntity.ok("kafka message sent 2");
    }
}

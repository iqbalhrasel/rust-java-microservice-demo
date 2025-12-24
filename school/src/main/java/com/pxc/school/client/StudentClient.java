package com.pxc.school.client;

import com.pxc.school.dtos.StudentDto;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.stereotype.Component;
import org.springframework.web.client.RestClient;

import java.util.List;

@Component
public class StudentClient {
    private final RestClient restClient;

    public StudentClient(RestClient.Builder builder) {
        this.restClient = builder
                .baseUrl("http://localhost:8090/students")
                .build();
    }

    public List<StudentDto> getAllStudentsBySchool(Integer schoolId){
        return restClient
                .get()
                .uri("/school/{school_id}", schoolId)
                .retrieve()
                .body(new ParameterizedTypeReference<>() {});
    }
}

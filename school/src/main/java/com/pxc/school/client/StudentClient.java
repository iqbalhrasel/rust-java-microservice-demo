package com.pxc.school.client;

import com.pxc.school.dtos.StudentDto;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.stereotype.Component;
import org.springframework.web.client.RestClient;
import org.springframework.web.client.RestTemplate;

import java.util.Arrays;
import java.util.List;

@Component
public class StudentClient {
    private final RestTemplate restTemplate;
    private final String BASE_URL = "http://student/students";

    public StudentClient(RestTemplate restTemplate) {
        this.restTemplate = restTemplate;
    }

    public List<StudentDto> getAllStudentsBySchool(Integer schoolId){
        var dtos = restTemplate.getForObject(BASE_URL+"/school/{schoolId}", StudentDto[].class, schoolId);
        return Arrays.asList(dtos);
    }
}

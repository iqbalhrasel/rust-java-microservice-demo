package com.pxc.school.dtos;

import lombok.Data;

import java.util.List;

@Data
public class SchoolResponse {
    private String name;
    private String email;
    private List<StudentDto> students;
}

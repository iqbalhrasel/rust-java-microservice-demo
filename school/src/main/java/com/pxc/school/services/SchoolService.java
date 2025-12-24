package com.pxc.school.services;

import com.pxc.school.client.StudentClient;
import com.pxc.school.dtos.SchoolDto;
import com.pxc.school.dtos.SchoolResponse;
import com.pxc.school.models.School;
import com.pxc.school.repositories.SchoolRepository;
import lombok.AllArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Service;

import java.util.List;

@AllArgsConstructor
@Service
public class SchoolService {
    private final SchoolRepository schoolRepository;
    private final StudentClient studentClient;

    public void saveSchool(School school){
        schoolRepository.save(school);
    }

    public List<School> getAllSchools(){
        return schoolRepository.findAll();
    }

    public SchoolResponse getSchoolWithStudents(Integer schoolId) {
        var school = schoolRepository.findById(schoolId).orElseThrow(()-> new RuntimeException("school not found"));

        var students = studentClient.getAllStudentsBySchool(schoolId);

        var response = new SchoolResponse();
        response.setName(school.getName());
        response.setEmail(school.getEmail());
        response.setStudents(students);

        return response;
    }

    public SchoolDto getSchool(Integer schoolId) {
        var school = schoolRepository
                .findById(schoolId)
                .orElseThrow(SchoolNotFoundException::new);
        return new SchoolDto(school.getId(), school.getName(), school.getEmail());
    }
}

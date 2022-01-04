package com.union.api.domain;

import lombok.Data;
import java.sql.Timestamp;

@Data
public class AdUserEntry {

    private Long id;
    private String account;
    private String password;
    private String name;
    private String avatar;
    private Integer status;
    private Integer isAdmin;
    private Timestamp createTime;
    private Timestamp updateTime;

    public AdUserEntry() {  }

}


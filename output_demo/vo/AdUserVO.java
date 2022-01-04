package com.union.api.vo;

import com.union.api.domain.AdUserEntry;
import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Data;
import java.sql.Timestamp;

@Data
public class AdUserVO {

    private Long id;
    private String account;
    private String password;
    private String name;
    private String avatar;
    private Integer status;
    @JsonProperty("is_admin")
    private Integer isAdmin;
    @JsonProperty("create_time")
    private Timestamp createTime;
    @JsonProperty("update_time")
    private Timestamp updateTime;

    public AdUserVO() {  }
 
    public AdUserVO(AdUserEntry adUser) {
        this.id = AdUserEntry.getId();
        this.account = AdUserEntry.getAccount();
        this.password = AdUserEntry.getPassword();
        this.name = AdUserEntry.getName();
        this.avatar = AdUserEntry.getAvatar();
        this.status = AdUserEntry.getStatus();
        this.isAdmin = AdUserEntry.getIsAdmin();
        this.createTime = AdUserEntry.getCreateTime();
        this.updateTime = AdUserEntry.getUpdateTime();
    }

}


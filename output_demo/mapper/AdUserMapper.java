package com.union.api.mapper;

import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;
import com.union.api.domain.AdUserEntry;
import com.union.api.dto.filter.AdUserFilter;
import java.util.List;
import lombok.Data;

@Mapper
public interface AdUserMapper {

    AdUserEntry getById(@Param("id") Long id);

    List<AdUserEntry> getByIds(@Param("ids") Long ids);

    List<AdUserEntry> getPageList(@Param("filter") AdUserFilter filter, @Param("limit") Integer limit, @Param("offset") Integer offset);

    Integer getCount(@Param("filter") AdUserFilter filter);

    List<AdUserEntry> getByFilter(@Param("filter") AdUserFilter filter);

    void add(@Param("adUser") AdUserEntry adUser);

    void update(@Param("adUser") AdUserEntry adUser);


}


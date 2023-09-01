use chrono::DateTime;
use serde_json::{json, Map, Value};

pub fn parse_ul(ulj: &str) -> Value {
    let mut map = Map::new();

    let mut ulj_splitted: Vec<&str> = ulj.split_terminator('|').collect();
    let rfc3339 = DateTime::parse_from_rfc3339(ulj_splitted[0]);
    if let Ok(_rfc3339) = &rfc3339 {
        map.insert(
            "TimeInstant".to_string(),
            Value::String(ulj_splitted[0].to_string()),
        );
        ulj_splitted = (&ulj_splitted[1..]).to_vec();
    }

    for i in (0..ulj_splitted.len()).step_by(2) {
        if let Ok(x) = ulj_splitted[i + 1].parse::<i64>() {
            map.insert(ulj_splitted[i].into(), x.into());
        } else if let Ok(x) = ulj_splitted[i + 1].parse::<f64>() {
            map.insert(ulj_splitted[i].into(), x.into());
        } else if ulj_splitted[i + 1].eq("null") {
            map.insert(ulj_splitted[i].into(), json!(null));
        } else {
            map.insert(ulj_splitted[i].into(), ulj_splitted[i + 1].into());
        }
    }

    let obj = Value::Object(map);
    return obj;
}
pub fn parse_ul_to_attribute(ulj: &str) -> Value {
    let mut attrs: Vec<Value> = Vec::new();

    let mut ulj_splitted: Vec<&str> = ulj.split_terminator('|').collect();
    let rfc3339 = DateTime::parse_from_rfc3339(ulj_splitted[0]);
    if let Ok(_rfc3339) = &rfc3339 {
        ulj_splitted = (&ulj_splitted[1..]).to_vec();
    }

    for i in (0..ulj_splitted.len()).step_by(2) {
        let token = ulj_splitted[i];
        let mut attr_map = Map::new();

        attr_map.insert("object_id".to_string(), token.into());
        if token.contains('.') {
            attr_map.insert(
                "name".to_string(),
                token.split_terminator('.').last().into(),
            );
        } else {
            attr_map.insert("name".to_string(), token.into());
        }

        if let Ok(_x) = ulj_splitted[i + 1].parse::<i64>() {
            attr_map.insert("type".to_string(), "Integer".into());
        } else if let Ok(_x) = ulj_splitted[i + 1].parse::<f64>() {
            attr_map.insert("type".to_string(), "Float".into());
        } else if ulj_splitted[i + 1].eq("null") {
            attr_map.insert("type".to_string(), "Float".into());
        } else {
            attr_map.insert("type".to_string(), "String".into());
        }

        attrs.push(Value::Object(attr_map));
    }

    let obj = Value::Array(attrs);
    return obj;
}

#[cfg(test)]
mod tests {
    use crate::{parse_ul, parse_ul_to_attribute};
    const ULJ_TIMESTAMP: &str = "2023-07-26T13:34:08Z|hp.r_cop|0.00|hp.r_prcond|15.40|hp.r_prevap|13.90|hp.r_prccp|0.00|hp.r_prchsp|0.00|hp.r_prcteev|0.00|hp.r_tempout1|17.00|hp.r_tempsucg|24.50|logvar.b_5|0|logvar.b_6|0|logvar.b_extwe|0|logvar.b_gsiv1|0|logvar.b_gsiv2|0|logvar.b_v_ww|0|logvar.di_ptherm_hzg|0|logvar.di_ptherm_wp|0|logvar.di_ptherm_ww|0|logvar.di_p_5|0|logvar.di_p_6|null|logvar.di_p_7|0|logvar.di_p_8|0|logvar.di_p_bat_ac|-9999|logvar.di_p_batt|0|logvar.di_p_batt_dc|-9999|logvar.di_p_batt_pump|-9999|logvar.di_p_batt_soc|99|logvar.di_p_booster|null|logvar.di_p_comp|0|logvar.di_p_eheiz|0|logvar.di_p_haus|431|logvar.di_p_netz|-4315|logvar.di_p_netz2|0|logvar.di_p_pheizung|null|logvar.di_p_pquelle|null|logvar.di_p_solar|4746|logvar.di_p_tww_ecopac|null|logvar.di_p_vent|null|logvar.di_p_w2|0|logvar.di_p_wp|0|logvar.i_2|0|logvar.i_3|0|logvar.i_batt_cyc|0|logvar.i_count_start_comp|10365";
    const ULJ: &str = "hp.r_cop|0.00|hp.r_prcond|15.40|hp.r_prevap|13.90|hp.r_prccp|0.00|hp.r_prchsp|0.00|hp.r_prcteev|0.00|hp.r_tempout1|17.00|hp.r_tempsucg|24.50|logvar.b_5|0|logvar.b_6|0|logvar.b_extwe|0|logvar.b_gsiv1|0|logvar.b_gsiv2|0|logvar.b_v_ww|0|logvar.di_ptherm_hzg|0|logvar.di_ptherm_wp|0|logvar.di_ptherm_ww|0|logvar.di_p_5|0|logvar.di_p_6|null|logvar.di_p_7|0|logvar.di_p_8|0|logvar.di_p_bat_ac|-9999|logvar.di_p_batt|0|logvar.di_p_batt_dc|-9999|logvar.di_p_batt_pump|-9999|logvar.di_p_batt_soc|99|logvar.di_p_booster|null|logvar.di_p_comp|0|logvar.di_p_eheiz|0|logvar.di_p_haus|431|logvar.di_p_netz|-4315|logvar.di_p_netz2|0|logvar.di_p_pheizung|null|logvar.di_p_pquelle|null|logvar.di_p_solar|4746|logvar.di_p_tww_ecopac|null|logvar.di_p_vent|null|logvar.di_p_w2|0|logvar.di_p_wp|0|logvar.i_2|0|logvar.i_3|0|logvar.i_batt_cyc|0|logvar.i_count_start_comp|10365";
    #[test]
    fn parse_ulj_with_timestamp() {
        parse_ul(ULJ_TIMESTAMP);
        parse_ul_to_attribute(ULJ_TIMESTAMP);
        assert!(true);
    }
    #[test]
    fn parse_ulj_without_timestamp() {
        parse_ul(ULJ);
        parse_ul_to_attribute(ULJ);
        assert!(true);
    }
}

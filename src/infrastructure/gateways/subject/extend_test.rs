#[cfg(test)]
mod test {
    use crate::domain::entities::{
        subject::{SubjectFixedSchedule, SubjectFlag, SubjectSchedule},
        SubjectEntity,
    };
    use crate::infrastructure::gateways::subject::dto::{
        SubjectDocument, SubjectFixedScheduleDoc, SubjectScheduleDoc,
    };
    use crate::utils::elasticsearch::GetResponse;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    fn subject_flag_test() {
        assert_eq!(
            SubjectFlag::from_str("internship").unwrap(),
            SubjectFlag::Internship
        );
        assert_eq!(
            SubjectFlag::from_str("lottery").unwrap(),
            SubjectFlag::Lottery
        );
        assert_eq!(SubjectFlag::from_str("kyoto").unwrap(), SubjectFlag::Kyoto);
        assert_eq!(SubjectFlag::from_str("univ3").unwrap(), SubjectFlag::Univ3);
        assert_eq!(SubjectFlag::from_str("pbl").unwrap(), SubjectFlag::PBL);
        assert_eq!(SubjectFlag::from_str("igp").unwrap(), SubjectFlag::IGP);
        assert_eq!(SubjectFlag::from_str("al").unwrap(), SubjectFlag::AL);
        assert_eq!(SubjectFlag::from_str("pt").unwrap(), SubjectFlag::PT);
        assert!(
            if let Err(_) = SubjectFlag::from_str("invalid") {
                true
            } else {
                false
            },
            "Invalid string should return Err"
        );
    }

    #[test]
    fn subject_schedule_test() {
        let actual = SubjectSchedule::try_from(SubjectScheduleDoc {
            schedule_type: String::from("intensive"),
            days: None,
        })
        .unwrap();
        assert_eq!(actual, SubjectSchedule::Intensive);

        let actual = SubjectSchedule::try_from(SubjectScheduleDoc {
            schedule_type: String::from("unknown"),
            days: None,
        })
        .unwrap();
        assert_eq!(actual, SubjectSchedule::Unknown);

        let actual = SubjectSchedule::try_from(SubjectScheduleDoc {
            schedule_type: String::from("fixed"),
            days: Some(vec![SubjectFixedScheduleDoc { date: 1, hour: 1 }]),
        })
        .unwrap();
        assert_eq!(
            actual,
            SubjectSchedule::Fixed(vec![SubjectFixedSchedule { date: 1, hour: 1 }])
        );

        let actual_err = SubjectSchedule::try_from(SubjectScheduleDoc {
            schedule_type: String::from("invalid"),
            days: None,
        });
        assert!(actual_err.is_err());

        let actual_err = SubjectSchedule::try_from(SubjectScheduleDoc {
            schedule_type: String::from("fixed"),
            days: None,
        });
        assert!(actual_err.is_err());
    }

    #[test]
    fn subject_entity_err_test() {
        let actual = SubjectEntity::try_from(GetResponse::<SubjectDocument> {
            found: false,
            _id: String::from("80"),
            _source: None,
        });
        assert!(actual.is_err());
    }

    #[test]
    fn subject_entity_ok_test() {
        let test_json = r#"
        {
            "_index" : "kittimetable_subjects",
            "_type" : "_doc",
            "_id" : "300",
            "_version" : 1,
            "_seq_no" : 789,
            "_primary_term" : 1,
            "found" : true,
            "_source" : {
              "id" : 300,
              "courseId" : 10061837,
              "credits" : 1,
              "flags" : [ ],
              "categories" : [
                {
                  "available" : true,
                  "year" : [
                    2
                  ],
                  "semester" : "前学期",
                  "faculty" : "全学共通科目",
                  "field" : "言語教育科目",
                  "program" : "英語",
                  "category" : "",
                  "schedule" : {
                    "type" : "unknown"
                  }
                }
              ],
              "type" : "演習",
              "title" : "Active English Reading Ⅱ",
              "instructors" : [
                {
                  "id" : null,
                  "name" : "(金丸　敏幸)"
                }
              ],
              "outline" : "outline",
              "purpose" : "purpose",
              "requirement" : "requirement",
              "point" : "point",
              "textbook" : "textbook",
              "gradingPolicy" : "gradingPolicy",
              "remark" : "remark",
              "researchPlan" : "",
              "plans" : [
                {
                  "topic" : "topic",
                  "content" : "content"
                }
              ],
              "goal" : {
                "description" : "description",
                "evaluations" : [
                  {
                    "label" : "label",
                    "description" : "description"
                  }
                ]
              }
            }
          }          
        "#;
        let test_parsed = serde_json::from_str::<GetResponse<SubjectDocument>>(test_json).unwrap();
        let actual = SubjectEntity::try_from(test_parsed);
        assert!(actual.is_ok());
    }
}

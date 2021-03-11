#[cfg(test)]
mod test {
    use crate::domain::entities::{
        subject::{SubjectFixedSchedule, SubjectFlag, SubjectSchedule},
        subject_td, SubjectEntity,
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
            "_id" : "1",
            "_version" : 1,
            "_seq_no" : 789,
            "_primary_term" : 1,
            "found" : true,
            "_source" : {
              "id" : 1,
              "courseId" : 3,
              "credits" : 4,
              "flags" : ["internship"],
              "class": "class_name",
              "code": "code",
              "categories" : [
                {
                  "available" : true,
                  "year" : [
                    1000
                  ],
                  "semester" : "semester",
                  "faculty" : "faculty",
                  "field" : "field",
                  "program" : "program",
                  "category" : "category",
                  "schedule" : {
                    "type" : "fixed",
                    "days" : [
                      {
                        "date" : 1,
                        "hour" : 1
                      }
                    ]
                  }
                },
                {
                  "available" : true,
                  "year" : [
                    1000
                  ],
                  "semester" : "semester",
                  "faculty" : "faculty",
                  "field" : "field",
                  "program" : "program",
                  "category" : "category",
                  "schedule" : {
                    "type" : "intensive"
                  }
                }
              ],
              "type" : "subject_type",
              "title" : "title",
              "instructors" : [
                {
                  "id" : "id1",
                  "name" : "ins1"
                },
                {
                  "id" : null,
                  "name" : "ins2"
                }
              ],
              "outline" : "outline",
              "purpose" : "purpose",
              "requirement" : "requirement",
              "point" : "point",
              "textbook" : "textbook",
              "gradingPolicy" : "grading_policy",
              "remark" : "remark",
              "researchPlan" : "research_plan",
              "timetableId": 2,
              "plans" : [
                {
                  "topic" : "topic",
                  "content" : "content"
                },
                {
                  "topic" : "topic",
                  "content" : null
                }
              ],
              "goal" : {
                "description" : "desc",
                "evaluations" : [
                  {
                    "label" : "label",
                    "description" : "desc2"
                  }
                ]
              },
              "attachments": [
                {
                  "key": "a1",
                  "name": "b1"
                }
              ]
            }
          }          
        "#;
        let test_parsed = serde_json::from_str::<GetResponse<SubjectDocument>>(test_json).unwrap();
        let actual = SubjectEntity::try_from(test_parsed);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), subject_td::get_subject_entity_test_data())
    }
}

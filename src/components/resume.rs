use leptos::*;
use serde::Deserialize;

#[component]
pub fn EmployerComponent(cx: Scope, employer: Employer) -> impl IntoView {
    view! {
        cx,
        <section class="jobDescription">
            <h3 class="jobTitle">{employer.name}</h3>
            {employer.positions.into_iter().map(|p| view! {cx,
                <div class="positionHeader">
                    <h4 class="jobDateRange">{p.job_title}</h4>
                    <h5 class="jobLocation">{p.date_start} - {p.date_end} | {p.city}, {p.state}</h5>
                </div>

                <div class="jobAccomplishments">
                    <ul>
                    {p.accomplishments.into_iter().map(|n| view! {
                        cx,
                        <li>{n}</li>
                    }).collect::<Vec<_>>()}
                    </ul>
                </div>
            }).collect::<Vec<_>>()}

        </section>
    }
}

#[component]
pub fn SkillComponent(cx: Scope, skill_name: String) -> impl IntoView {
    view! {
        cx,
        <li class="skillsListItem">{skill_name}</li>
    }
}

#[component]
pub fn SkillzComponent(cx: Scope, skillz: Vec<String>) -> impl IntoView {
    view! {
        cx,
        <section class="skillsComponent">
            <h2>"Skills"</h2>
            <ul class="skillsList">
                {skillz.into_iter().map(|n| view! {
                    cx,
                    <SkillComponent skill_name={n} />
                }).collect::<Vec<_>>()}
            </ul>
        </section>

    }
}

#[component]
pub fn DegreeItemComponent(cx: Scope, d: EdDegree) -> impl IntoView {
    view! {
        cx,
            <h4 class="educationalDegreeInstitution">{d.institution}</h4>
            <ul><li>{d.degree} | {d.graduation_date}</li></ul>
    }
}

#[component]
pub fn DegreeListComponent(cx: Scope, degreez: Vec<EdDegree>) -> impl IntoView {
    view! {
        cx,
        <section class="degreeListComponent">
            <h2>"Education"</h2>
            <div class="degreeList">
                {degreez.into_iter().map(|n| view! {
                    cx,
                    <DegreeItemComponent d={n} />
                }).collect::<Vec<_>>()}
            </div>
        </section>

    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub city: String,
    pub state: String,
    pub job_title: String,
    pub date_start: String,
    pub date_end: String,
    pub accomplishments: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employer {
    pub name: String,
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdDegree {
    pub institution: String,
    pub degree: String,
    pub graduation_date: String,
}

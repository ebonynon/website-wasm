use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct Contribution {
    pub name: &'static str,
    pub url: &'static str,
    pub icon_url: &'static str,
    pub description: &'static str,
    pub languages: &'static [&'static str],
    pub html: fn() -> Html,
}

pub type ContributionListType = [Contribution; 4];

pub static CONTRIBUTION_LIST: ContributionListType = [
    Contribution {
        name: "Capstone",
        url: "https://github.com/aquynh/capstone",
        icon_url: "http://www.capstone-engine.org/img/capstone.png",
        description: "Multi-platform, multi-architecture disassembly framework",
        languages: &["C"],
        html: || {
            html! {
            <ul>
                <li>{ "Add build support for IBM platforms such as AIX and zOS" }</li>
                <li>{ "Fix zOS instruction disassembly for instructions where base is 0 but index is not" }</li>
            </ul>
                    }
        },
    },
    Contribution {
        name: "Portage",
        url: "https://github.com/gentoo/gentoo",
        icon_url: "https://upload.wikimedia.org/wikipedia/commons/4/41/Gentoo-logo-dark.svg",
        description: "Official package manager and distribution system for Gentoo",
        languages: &["Shell"],
        html: || {
            html! {
            <ul>
                <li>{ "Package maintainer for " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/easymodo/qimgv">{ "qimgv" }</a></li>
            </ul>
                    }
        },
    },
    Contribution {
        name: "Maven",
        url: "https://github.com/carlossg/docker-maven",
        icon_url: "http://cdn.onlinewebfonts.com/svg/img_161017.png",
        description: "Maven Dockerfiles",
        languages: &["Scripting"],
        html: || {
            html! {
            <ul>
                <li>{ "Add Dockerfile with " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://www.eclipse.org/openj9/">{ "OpenJ9" }</a>{ " as Java implementation" }</li>
            </ul>
                    }
        },
    },
    Contribution {
        name: "Jenkins",
        url: "https://github.com/jenkinsci/jenkins",
        icon_url: "https://jenkins.io/images/logos/jenkins/256.png",
        description: "Free and open source automation server",
        languages: &["Scripting"],
        html: || {
            html! {
            <ul>
                <li>{ "Add Dockerfile with " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://www.eclipse.org/openj9/">{ "OpenJ9" }</a>{ " as Java implementation
                with " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://www.eclipse.org/openj9/docs/shrc/">{ "Shared Classes Cache" }</a>{ " enabled" }</li>
                <li>{ "Decrease memory usage by up to 40% and decrease startup times by up to 30%" }</li>
            </ul>
                    }
        },
    },
];

use derive_setters::Setters;
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;

use crate::{is_default, SetEvent};

#[derive(Serialize, Setters, Clone)]
#[serde(rename_all = "snake_case")]
#[setters(strip_option)]
pub struct Event {
    #[serde(skip_serializing_if = "is_default")]
    pub push: Option<Push>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "is_default")]
    pub branch_protection_rule: Option<BranchProtectionRule>,
    #[serde(skip_serializing_if = "is_default")]
    pub check_run: Option<CheckRun>,
    #[serde(skip_serializing_if = "is_default")]
    pub check_suite: Option<CheckSuite>,
    #[serde(skip_serializing_if = "is_default")]
    pub create: Option<Create>,
    #[serde(skip_serializing_if = "is_default")]
    pub delete: Option<Delete>,
    #[serde(skip_serializing_if = "is_default")]
    pub deployment: Option<Deployment>,
    #[serde(skip_serializing_if = "is_default")]
    pub deployment_status: Option<DeploymentStatus>,
    #[serde(skip_serializing_if = "is_default")]
    pub discussion: Option<Discussion>,
    #[serde(skip_serializing_if = "is_default")]
    pub discussion_comment: Option<DiscussionComment>,
    #[serde(skip_serializing_if = "is_default")]
    pub fork: Option<Fork>,
    #[serde(skip_serializing_if = "is_default")]
    pub gollum: Option<Gollum>,
    #[serde(skip_serializing_if = "is_default")]
    pub issue_comment: Option<IssueComment>,
    #[serde(skip_serializing_if = "is_default")]
    pub issues: Option<Issues>,
    #[serde(skip_serializing_if = "is_default")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "is_default")]
    pub merge_group: Option<MergeGroup>,
    #[serde(skip_serializing_if = "is_default")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "is_default")]
    pub page_build: Option<PageBuild>,
    #[serde(skip_serializing_if = "is_default")]
    pub project: Option<Project>,
    #[serde(skip_serializing_if = "is_default")]
    pub project_card: Option<ProjectCard>,
    #[serde(skip_serializing_if = "is_default")]
    pub project_column: Option<ProjectColumn>,
    #[serde(skip_serializing_if = "is_default")]
    pub public: Option<Public>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request_review: Option<PullRequestReview>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request_review_comment: Option<PullRequestReviewComment>,
    #[serde(skip_serializing_if = "is_default")]
    pub pull_request_target: Option<PullRequestTarget>,
    #[serde(skip_serializing_if = "is_default")]
    pub registry_package: Option<RegistryPackage>,
    #[serde(skip_serializing_if = "is_default")]
    pub release: Option<Release>,
    #[serde(skip_serializing_if = "is_default")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "is_default")]
    pub watch: Option<Watch>,
    #[serde(skip_serializing_if = "is_default")]
    pub workflow_call: Option<WorkflowCall>,
    #[serde(skip_serializing_if = "is_default")]
    pub workflow_dispatch: Option<WorkflowDispatch>,
    #[serde(skip_serializing_if = "is_default")]
    pub workflow_run: Option<WorkflowRun>,
    #[serde(skip_serializing_if = "is_default")]
    pub repository_dispatch: Option<RepositoryDispatch>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BranchActivity {
    Created,
    #[default]
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunActivity {
    #[default]
    Created,
    #[serde(rename = "rerequested")]
    ReRequested,
    Completed,
    RequestedAction,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteActivity {
    #[default]
    Completed,
    Requested,
    #[serde(rename = "rerequested")]
    ReRequested,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestActivity {
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    #[default]
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
    ConvertedToDraft,
    ReadyForReview,
    Locked,
    Unlocked,
    Milestoned,
    Demilestoned,
    ReviewRequested,
    ReviewRequestRemoved,
    AutoMergeEnabled,
    AutoMergeDisabled,
    Enqueued,
    Dequeued,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionActivity {
    #[default]
    Created,
    Edited,
    Deleted,
    Transferred,
    Pinned,
    Unpinned,
    Labeled,
    Unlabeled,
    Locked,
    Unlocked,
    CategoryChanged,
    Answered,
    Unanswered,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionCommentActivity {
    #[default]
    Created,
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentActivity {
    #[default]
    Created,
    Edited,
    Deleted,
}
#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuesActivity {
    #[default]
    Opened,
    Edited,
    Deleted,
    Transferred,
    Pinned,
    Unpinned,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Locked,
    Unlocked,
    Milestoned,
    Demilestoned,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LabelActivity {
    #[default]
    Created,
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MergeGroupActivity {
    #[default]
    ChecksRequested,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneActivity {
    #[default]
    Created,
    Closed,
    Opened,
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectActivity {
    #[default]
    Created,
    Updated,
    Closed,
    Reopened,
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCardActivity {
    #[default]
    Created,
    Moved,
    Converted,
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectColumnActivity {
    #[default]
    Created,
    Updated,
    Moved,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewActivity {
    #[default]
    Submitted,
    Edited,
    Dismissed,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentActivity {
    #[default]
    Created,
    Edited,
    Deleted,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestTargetActivity {
    #[default]
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
    ConvertedToDraft,
    ReadyForReview,
    Locked,
    Unlocked,
    Milestoned,
    Demilestoned,
    ReviewRequested,
    ReviewRequestRemoved,
    AutoMergeEnabled,
    AutoMergeDisabled,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryPackageActivity {
    #[default]
    Published,
    Updated,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseActivity {
    #[default]
    Published,
    Unpublished,
    Created,
    Edited,
    Deleted,
    Prereleased,
    Released,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunActivity {
    #[default]
    Requested,
    Completed,
    InProgress,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Push {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Public;

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequest {
    types: Vec<PullRequestActivity>,
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct BranchProtectionRule {
    types: Vec<BranchActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CheckRun {
    types: Vec<CheckRunActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CheckSuite {
    types: Vec<CheckSuiteActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Create {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Delete {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Deployment {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentStatus {
    branches: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Discussion {
    types: Vec<DiscussionActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DiscussionComment {
    types: Vec<DiscussionCommentActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Fork;

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Gollum;

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct IssueComment {
    types: Vec<IssueCommentActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Issues {
    types: Vec<IssuesActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Label {
    types: Vec<LabelActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct MergeGroup {
    types: Vec<MergeGroupActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Milestone {
    types: Vec<MilestoneActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PageBuild;

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    types: Vec<ProjectActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectCard {
    types: Vec<ProjectCardActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectColumn {
    types: Vec<ProjectColumnActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestReview {
    types: Vec<PullRequestReviewActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestReviewComment {
    types: Vec<PullRequestReviewCommentActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestTarget {
    types: Vec<PullRequestTargetActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RegistryPackage {
    types: Vec<RegistryPackageActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    types: Vec<ReleaseActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Status;

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Watch {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    types: Vec<WatchActivity>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WatchActivity {
    #[default]
    Started,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowCall;

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowDispatch {
    pub inputs: IndexMap<String, Value>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowRun {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    types: Vec<WorkflowRunActivity>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    workflows: Vec<String>,
}

#[derive(Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryDispatch {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    types: Vec<String>,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            push: Some(Push::default()),
            pull_request: None,
            branch_protection_rule: None,
            check_run: None,
            check_suite: None,
            create: None,
            delete: None,
            deployment: None,
            deployment_status: None,
            discussion: None,
            discussion_comment: None,
            fork: None,
            gollum: None,
            issue_comment: None,
            issues: None,
            label: None,
            merge_group: None,
            milestone: None,
            page_build: None,
            project: None,
            project_card: None,
            project_column: None,
            public: None,
            pull_request_review: None,
            pull_request_review_comment: None,
            pull_request_target: None,
            registry_package: None,
            release: None,
            status: None,
            watch: None,
            workflow_call: None,
            workflow_dispatch: None,
            workflow_run: None,
            repository_dispatch: None,
        }
    }
}

impl SetEvent for Event {
    fn apply(self, mut workflow: crate::Workflow) -> crate::Workflow {
        workflow.on = serde_json::to_value(self).ok();
        workflow
    }
}

impl Push {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }
}

impl PullRequest {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }

    pub fn open(mut self) -> Self {
        self.types.push(PullRequestActivity::Opened);
        self
    }

    pub fn synchronize(mut self) -> Self {
        self.types.push(PullRequestActivity::Synchronize);
        self
    }

    pub fn reopen(mut self) -> Self {
        self.types.push(PullRequestActivity::Reopened);
        self
    }
}

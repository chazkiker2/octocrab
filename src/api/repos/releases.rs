use super::*;

/// Handler for GitHub's releases API.
///
/// Created with [`RepoHandler::releases`].
///
/// [`RepoHandler::releases`]: ../struct.RepoHandler.html#method.releases
pub struct ReleasesHandler<'octo, 'r> {
    parent: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> ReleasesHandler<'octo, 'r> {
    pub(crate) fn new(parent: &'r RepoHandler<'octo>) -> Self {
        Self { parent }
    }

    /// Creates a new `ListReleasesBuilder` that can be configured to filter
    /// listing releases.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let page = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .list()
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListReleasesBuilder<'_, '_, '_> {
        ListReleasesBuilder::new(self)
    }

    /// Creates a new `CreateReleaseBuilder` with `tag_name`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let page = octocrab.repos("owner", "repo")
    ///     .releases()
    ///     .create("v1.0.0")
    ///     // Optional Parameters
    ///     .target_commitish("main")
    ///     .name("Version 1.0.0")
    ///     .body("Announcing 1.0.0!")
    ///     .draft(false)
    ///     .prerelease(false)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<'t>(&self, tag_name: &'t (impl AsRef<str> + ?Sized)) -> CreateReleaseBuilder<'_, '_, '_, 't, '_, '_, '_> {
        CreateReleaseBuilder::new(self, tag_name.as_ref())
    }
}

/// A builder pattern struct for listing releases.
///
/// created by [`ReleasesHandler::list`]
///
/// [`PullRequestHandler::list`]: ./struct.ReleasesHandler.html#method.list
#[derive(serde::Serialize)]
pub struct ListReleasesBuilder<'octo, 'r1, 'r2> {
    #[serde(skip)]
    handler: &'r2 ReleasesHandler<'octo, 'r1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r1, 'r2> ListReleasesBuilder<'octo, 'r1, 'r2> {
    pub(crate) fn new(handler: &'r2 ReleasesHandler<'octo, 'r1>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::repos::Release>> {
        let url = format!(
            "/repos/{owner}/{repo}/releases",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.crab.get(url, Some(&self)).await
    }
}

/// A builder pattern struct for listing releases.
///
/// created by [`ReleasesHandler::list`]
///
/// [`PullRequestHandler::list`]: ./struct.ReleasesHandler.html#method.list
#[derive(serde::Serialize)]
pub struct CreateReleaseBuilder<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body> {
    #[serde(skip)]
    handler: &'handler ReleasesHandler<'octo, 'repos>,
    tag_name: &'tag_name str,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<&'target_commitish str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'name str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'body str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prerelease: Option<bool>,
}

impl<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body> CreateReleaseBuilder<'octo, 'repos, 'handler, 'tag_name, 'target_commitish, 'name, 'body> {
    pub(crate) fn new(handler: &'handler ReleasesHandler<'octo, 'repos>, tag_name: &'tag_name str) -> Self {
        Self {
            handler,
            tag_name,
            target_commitish: None,
            name: None,
            body: None,
            draft: None,
            prerelease: None,
        }
    }

    /// Specifies the commitish value that determines where the Git tag is
    /// created from. Can be any branch or commit SHA. Unused if the Git tag
    /// already exists. Default: the repository's default branch
    /// (usually `main`).
    pub fn target_commitish(mut self, target_commitish: &'target_commitish (impl AsRef<str> + ?Sized)) -> Self {
        self.target_commitish = Some(target_commitish.as_ref());
        self
    }

    /// The name of the release.
    pub fn name(mut self, name: &'name (impl AsRef<str> + ?Sized)) -> Self {
        self.name = Some(name.as_ref());
        self
    }

    /// Text describing the contents of the tag.
    pub fn body(mut self, body: &'body (impl AsRef<str> + ?Sized)) -> Self {
        self.body = Some(body.as_ref());
        self
    }

    /// Whether to set the release as a "draft" release or not.
    pub fn draft(mut self, draft: impl Into<bool>) -> Self {
        self.draft = Some(draft.into());
        self
    }

    /// Whether to set the release as a "prerelease" or not.
    pub fn prerelease(mut self, prerelease: impl Into<bool>) -> Self {
        self.prerelease = Some(prerelease.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::Release> {
        let url = format!(
            "/repos/{owner}/{repo}/releases",
            owner = self.handler.parent.owner,
            repo = self.handler.parent.repo
        );
        self.handler.parent.crab.get(url, Some(&self)).await
    }
}

use crate::common::Result;
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use uuid::Uuid;

pub trait UserRepository: Send + Sync {
    /// Creates a new user record in the repository with the provided `UserInformation`.
    ///
    /// # Parameters
    /// - `user_information`: A reference to the `UserInformation` struct containing the user's details.
    ///
    /// # Returns
    /// A `Result` wrapping `UserInformation` of the created user, or an error if the operation fails.
    ///
    /// # Errors
    /// This function will return an error if the user information is invalid or if the database operation fails.
    fn create(&self, user_information: &UserInformation) -> Result<UserInformation>;

    /// Retrieves a user's information by their unique identifier.
    ///
    /// # Parameters
    /// - `user_id`: A reference to the `Uuid` of the user to retrieve.
    ///
    /// # Returns
    /// A `Result` wrapping `UserInformation` if found, or an error if no user is found with the given ID or if a database error occurs.
    ///
    /// # Errors
    /// This function will return an error if the user ID does not exist or if a database error occurs.
    fn get(&self, user_id: &Uuid) -> Result<UserInformation>;

    /// Retrieves a user's information by their username.
    ///
    /// # Parameters
    /// - `user_name`: A reference to the `str` of the username to retrieve.
    ///
    /// # Returns
    /// A `Result` wrapping `UserInformation` if found, or an error if no user is found with the given username or if a database error occurs.
    ///
    /// # Errors
    /// This function will return an error if the username does not exist or if a database error occurs.
    fn get_by_name(&self, user_name: &str) -> Result<UserInformation>;

    /// Sets or updates the profile picture URL for a specified user.
    ///
    /// # Parameters
    /// - `user_id`: A reference to the `Uuid` of the user for whom the profile picture is being set.
    /// - `profile_picture`: A string slice representing the URL of the new profile picture.
    ///
    /// # Returns
    /// A `Result` wrapping `UserInformation` of the user with updated profile picture, or an error if the operation fails.
    ///
    /// # Errors
    /// This function will return an error if the user ID does not exist, or the provided URL is not valid, or if the database operation fails.
    fn set_profile_picture(&self, user_id: &Uuid, profile_picture: &str)
        -> Result<UserInformation>;

    /// Authenticates a user based on their login credentials.
    ///
    /// # Parameters
    /// - `login_information`: A reference to the `LoginInformation` struct containing the user's login details.
    ///
    /// # Returns
    /// A `Result` wrapping `UserInformation` if the credentials are valid and the user is authenticated, or an error if authentication fails.
    ///
    /// # Errors
    /// This function will return an error if the login information is incorrect (e.g., wrong username or password) or if a database error occurs during authentication.
    fn login(&self, login_information: &LoginInformation) -> Result<UserInformation>;
}

class AddJointError(Exception): ...
class AddLinkError(Exception): ...

class GroupIDError(Exception):
    """An error which can be returned when checking for a `GroupID`'s validity. This error is used as an error type for functions which check for `GroupID` validity such as Link::is_valid_group_id

    TODO: DOCStyle
    """

    ...

class XMLError(Exception): ...
class RebuildBranchError(Exception): ...

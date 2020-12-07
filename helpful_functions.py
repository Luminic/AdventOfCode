def group_by(iterable, split, inclusive=False, join_as_str=False, discard_empty=False, max_splits=-1):
    """
    Input:

    inclusive: `False`, `"front"`, `"back"`, `"own_group"`
    how should the splitting element should be included in the result

    join_as_str: bool
    should each group should be converted into a string with "".join(elem)

    discard_empty: bool
    should an empty group be discarded

    max_splits: int
    the maximum number of splits allowed. This will result in max_splits+1 groups unless inclusive="own_group". A negative value will be ignored

    Output:
    A list of lists of the original elements grouped together based on split
    """
    assert inclusive == False or inclusive == "front" or inclusive == "back" or inclusive == "own_group"
    
    if not callable(split): split_on_element = lambda v: v == split
    else: split_on_element = split

    res = []

    current_group = []
    for elem in iterable:
        if split_on_element(elem) and max_splits != 0:
            max_splits -= 1

            if inclusive == 'front':
                current_group.append(elem)
            
            if (not discard_empty) or len(current_group) != 0:
                res.append(current_group)
            current_group = []

            if inclusive == 'front' or inclusive == False:
                continue
            elif inclusive == 'own_group':
                res.append(elem)
                continue

        current_group.append(elem)

    res.append(current_group)
    if join_as_str:
        for i, line in enumerate(res):
            res[i] = "".join(line)

    return res


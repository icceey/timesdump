using Windows.ApplicationModel.Resources;

namespace Timesdump.Services;

public static class LocalizationHelper
{
    private static ResourceLoader? _resourceLoader;

    private static ResourceLoader ResourceLoader
    {
        get
        {
            _resourceLoader ??= new ResourceLoader();
            return _resourceLoader;
        }
    }

    public static string GetString(string resourceKey)
    {
        try
        {
            return ResourceLoader.GetString(resourceKey);
        }
        catch
        {
            return resourceKey;
        }
    }
}

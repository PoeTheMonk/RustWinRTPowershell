using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Management.Automation;
using rs_winrt_sample;

namespace rs_winrt_sample_cs_ps
{
    #region GetBasicTestCommand

    /// <summary>
    /// This class implements the Get-BasicTest cmdlet.
    /// </summary>
    [Cmdlet(VerbsCommon.Get, "BasicTest")]
    public class GetBasicTestCommand : Cmdlet
    {
        #region Cmdlet Overrides

        /// <summary>
        /// </summary>
        protected override void ProcessRecord()
        {

            var basicTest = (new rs_winrt_sample.BasicTest("TEST"));

            WriteObject(basicTest);
        }

        #endregion Overrides

    } //GetBasicTestCommand

    #endregion GetBasicTestCommand

    #region GetBasicTestDatabaseCommand

    /// <summary>
    /// This class implements the Get-BasicTestDatabase cmdlet.
    /// </summary>
    [Cmdlet(VerbsCommon.Get, "BasicTestDatabase")]
    public class GetBasicTestDatabaseCommand : Cmdlet
    {
        #region Parameters

        /// <summary>
        /// Gets or sets the the number of items to return.
        /// </summary>
        [Parameter(Position = 0)]
        [ValidateNotNullOrEmpty]
        public uint Number
        {
            get;
            set;
        } = 1;

        #endregion Parameters

        #region Cmdlet Overrides

        /// <summary>
        /// Returns `Number` of BasicTest objects in an array to the pipeline.
        /// </summary>
        protected override void ProcessRecord()
        {
            if (Number == 0)
            {
                throw new PSArgumentException("Number must be greater than 0");
            }
            var basicTest = (new rs_winrt_sample.BasicTestDatabase()).GetCollection(Number);

            WriteObject(basicTest, true);
        }

        #endregion Overrides

    } //GetBasicTestCommand

    #endregion GetBasicTestCommand
}
